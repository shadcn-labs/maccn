#include <stdint.h>

static void* memset(void* s, int c, unsigned long n) {
    uint8_t* p = (uint8_t*)s;
    while (n--) *p++ = (uint8_t)c;
    return s;
}

static void* memcpy(void* dest, const void* src, unsigned long n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    while (n--) *d++ = *s++;
    return dest;
}

#define WASM_EXPORT __attribute__((visibility("default")))
#define INITIAL_PAGES 256
#define PAGE_SIZE 65536
#define MEM_LEN (INITIAL_PAGES * PAGE_SIZE)
#define HEAP_START (1024 * 1024)
#define LOGICAL_W 320
#define LOGICAL_H 200
#define MAX_INSTANCES 16
#define GLYPH_W 5
#define GLYPH_H 6
#define GLYPH_SPACING 1

typedef struct { uint8_t r, g, b, a; } RGBA;

static const RGBA LIGHT_BG = {255,255,255,255};
static const RGBA LIGHT_FG = {0,0,0,255};
static const RGBA LIGHT_ACCENT = {0,122,255,255};
static const RGBA LIGHT_BORDER = {216,216,220,255};
static const RGBA LIGHT_BG2 = {240,240,245,255};
static const RGBA LIGHT_GREEN = {52,199,89,255};
static const RGBA LIGHT_RED = {255,59,48,255};

static const RGBA DARK_BG = {30,30,30,255};
static const RGBA DARK_FG = {255,255,255,255};
static const RGBA DARK_ACCENT = {10,132,255,255};
static const RGBA DARK_BORDER = {55,55,55,255};
static const RGBA DARK_BG2 = {44,44,46,255};
static const RGBA DARK_GREEN = {48,209,88,255};
static const RGBA DARK_RED = {255,69,58,255};

static const RGBA WHITE = {255,255,255,255};

static uint32_t bump_ptr = HEAP_START;
static uint32_t render_buf_ptr = 0;

typedef struct {
    uint8_t name[64];
    uint32_t name_len;
    uint32_t dark;
    int64_t now_ms;
    uint32_t cursor;
    float scroll_x, scroll_y;
    float ptr_x, ptr_y;
    uint32_t ptr_down;
    uint32_t checkbox_checked, checkbox_indeterminate;
    uint32_t switch_on;
    float progress_value, slider_value;
    uint8_t radio_selected;
    uint8_t text_field_buf[64];
    uint32_t text_field_len;
    uint8_t secure_field_buf[64];
    uint32_t secure_field_len;
    uint32_t select_open;
    uint8_t select_index;
    uint32_t popup_open;
    uint8_t popup_index;
    uint8_t segmented_index;
    int32_t stepper_value;
    uint8_t search_buf[64];
    uint32_t search_len;
} Instance;

static Instance instances[MAX_INSTANCES];
static uint32_t instance_count = 0;

static uint32_t bump_alloc(uint32_t len) {
    uint32_t aligned = (bump_ptr + 7) & ~7u;
    if (aligned + len > MEM_LEN) return 0;
    bump_ptr = aligned + len;
    return aligned;
}

static uint8_t* wasm_mem(void) { return (uint8_t*)0; }

static void put_pixel(uint8_t* buf, int w, int x, int y, RGBA c) {
    if (x < 0 || y < 0 || x >= w) return;
    int h = (LOGICAL_W * LOGICAL_H * 4) / (w * 4);
    if (y >= h) return;
    int off = (y * w + x) * 4;
    buf[off] = c.r; buf[off+1] = c.g; buf[off+2] = c.b; buf[off+3] = c.a;
}

static void fill_rect(uint8_t* buf, int w, int x0, int y0, int x1, int y1, RGBA c) {
    for (int y = y0; y <= y1; y++)
        for (int x = x0; x <= x1; x++)
            put_pixel(buf, w, x, y, c);
}

static int iabs(int x) { return x < 0 ? -x : x; }

static void fill_rounded_rect(uint8_t* buf, int w, int bx0, int by0, int bx1, int by1, int r, RGBA c) {
    for (int y = by0; y <= by1; y++) {
        for (int x = bx0; x <= bx1; x++) {
            int inside = 1;
            if (x < bx0 || x > bx1 || y < by0 || y > by1) { inside = 0; }
            else if (r > 0) {
                if (x < bx0+r && y < by0+r) {
                    int dx = bx0+r-x, dy = by0+r-y;
                    if (dx*dx+dy*dy > r*r) inside = 0;
                } else if (x > bx1-r && y < by0+r) {
                    int dx = x-(bx1-r), dy = by0+r-y;
                    if (dx*dx+dy*dy > r*r) inside = 0;
                } else if (x < bx0+r && y > by1-r) {
                    int dx = bx0+r-x, dy = y-(by1-r);
                    if (dx*dx+dy*dy > r*r) inside = 0;
                } else if (x > bx1-r && y > by1-r) {
                    int dx = x-(bx1-r), dy = y-(by1-r);
                    if (dx*dx+dy*dy > r*r) inside = 0;
                }
            }
            if (inside) put_pixel(buf, w, x, y, c);
        }
    }
}

static void fill_circle(uint8_t* buf, int w, int cx, int cy, int radius, RGBA c) {
    for (int y = cy-radius; y <= cy+radius; y++)
        for (int x = cx-radius; x <= cx+radius; x++) {
            int dx = x-cx, dy = y-cy;
            if (dx*dx+dy*dy <= radius*radius)
                put_pixel(buf, w, x, y, c);
        }
}

static void fill_hline(uint8_t* buf, int w, int x0, int x1, int y, RGBA c) {
    for (int x = x0; x <= x1; x++) put_pixel(buf, w, x, y, c);
}

static void fill_vline(uint8_t* buf, int w, int x, int y0, int y1, RGBA c) {
    for (int y = y0; y <= y1; y++) put_pixel(buf, w, x, y, c);
}

static RGBA color_alpha(RGBA c, uint8_t a) { RGBA r = {c.r, c.g, c.b, a}; return r; }

static const uint8_t DIGIT_GLYPHS[10][6] = {
    {0x0E,0x11,0x13,0x15,0x19,0x0E},
    {0x04,0x0C,0x04,0x04,0x04,0x0E},
    {0x0E,0x11,0x02,0x04,0x08,0x1F},
    {0x0E,0x11,0x06,0x01,0x11,0x0E},
    {0x02,0x06,0x0A,0x12,0x1F,0x02},
    {0x1F,0x10,0x1E,0x01,0x11,0x0E},
    {0x06,0x08,0x1E,0x11,0x11,0x0E},
    {0x1F,0x01,0x02,0x04,0x04,0x04},
    {0x0E,0x11,0x0E,0x11,0x11,0x0E},
    {0x0E,0x11,0x11,0x0F,0x01,0x0E}
};

static const uint8_t LETTER_GLYPHS[26][6] = {
    {0x0E,0x11,0x11,0x1F,0x11,0x11},
    {0x1E,0x11,0x1E,0x11,0x11,0x1E},
    {0x0E,0x11,0x10,0x10,0x11,0x0E},
    {0x1C,0x12,0x11,0x11,0x12,0x1C},
    {0x1F,0x10,0x1E,0x10,0x10,0x1F},
    {0x1F,0x10,0x1E,0x10,0x10,0x10},
    {0x0E,0x11,0x10,0x13,0x11,0x0E},
    {0x11,0x11,0x1F,0x11,0x11,0x11},
    {0x0E,0x04,0x04,0x04,0x04,0x0E},
    {0x01,0x01,0x01,0x01,0x11,0x0E},
    {0x11,0x12,0x14,0x18,0x14,0x12},
    {0x10,0x10,0x10,0x10,0x10,0x1F},
    {0x11,0x1B,0x15,0x11,0x11,0x11},
    {0x11,0x19,0x15,0x13,0x11,0x11},
    {0x0E,0x11,0x11,0x11,0x11,0x0E},
    {0x1E,0x11,0x11,0x1E,0x10,0x10},
    {0x0E,0x11,0x11,0x15,0x12,0x0D},
    {0x1E,0x11,0x11,0x1E,0x14,0x12},
    {0x0E,0x11,0x0E,0x01,0x11,0x0E},
    {0x1F,0x04,0x04,0x04,0x04,0x04},
    {0x11,0x11,0x11,0x11,0x11,0x0E},
    {0x11,0x11,0x11,0x11,0x0A,0x04},
    {0x11,0x11,0x11,0x15,0x1B,0x11},
    {0x11,0x11,0x0A,0x04,0x0A,0x11},
    {0x11,0x11,0x0A,0x04,0x04,0x04},
    {0x1F,0x01,0x02,0x04,0x08,0x1F}
};

static const uint8_t SPACE_GLYPH[6] = {0,0,0,0,0,0};

static void draw_glyph(uint8_t* buf, int buf_w, int gx, int gy, const uint8_t* glyph, int scale, RGBA c) {
    for (int row = 0; row < GLYPH_H; row++) {
        uint8_t bits = glyph[row];
        for (int col = 0; col < GLYPH_W; col++) {
            uint8_t mask = 1 << (4 - col);
            if (bits & mask) {
                for (int sy = 0; sy < scale; sy++)
                    for (int sx = 0; sx < scale; sx++)
                        put_pixel(buf, buf_w, gx + col*scale + sx, gy + row*scale + sy, c);
            }
        }
    }
}

static void draw_digit(uint8_t* buf, int bw, int x, int y, uint8_t d, int s, RGBA c) {
    if (d <= 9) draw_glyph(buf, bw, x, y, DIGIT_GLYPHS[d], s, c);
}

static void draw_number(uint8_t* buf, int bw, int x, int y, uint32_t num, int s, RGBA c) {
    if (num == 0) { draw_digit(buf, bw, x, y, 0, s, c); return; }
    uint32_t tmp = num; int count = 0;
    while (tmp > 0) { count++; tmp /= 10; }
    int dx = (count - 1) * (GLYPH_W + GLYPH_SPACING) * s;
    tmp = num;
    while (tmp > 0) {
        draw_digit(buf, bw, x + dx, y, tmp % 10, s, c);
        dx -= (GLYPH_W + GLYPH_SPACING) * s;
        tmp /= 10;
    }
}

static void draw_letter(uint8_t* buf, int bw, int x, int y, uint8_t ch, int s, RGBA c) {
    if (ch == ' ') draw_glyph(buf, bw, x, y, SPACE_GLYPH, s, c);
    else if (ch >= 'A' && ch <= 'Z') draw_glyph(buf, bw, x, y, LETTER_GLYPHS[ch-'A'], s, c);
    else if (ch >= 'a' && ch <= 'z') draw_glyph(buf, bw, x, y, LETTER_GLYPHS[ch-'a'], s, c);
    else if (ch >= '0' && ch <= '9') draw_digit(buf, bw, x, y, ch-'0', s, c);
}

static void draw_string(uint8_t* buf, int bw, int x, int y, const char* text, int len, int s, RGBA c) {
    int cx = x;
    for (int i = 0; i < len; i++) {
        draw_letter(buf, bw, cx, y, (uint8_t)text[i], s, c);
        cx += (GLYPH_W + GLYPH_SPACING) * s;
    }
}

static void draw_bordered_rounded_rect(uint8_t* buf, int w, int x0, int y0, int x1, int y1, int r, RGBA fill, RGBA stroke) {
    for (int y = y0; y <= y1; y++) {
        for (int x = x0; x <= x1; x++) {
            int inside = 0;
            if (x >= x0 && x <= x1 && y >= y0 && y <= y1) {
                inside = 1;
                if (r > 0) {
                    if (x < x0+r && y < y0+r) {
                        int dx = x0+r-x, dy = y0+r-y;
                        if (dx*dx+dy*dy > r*r) inside = 0;
                    } else if (x > x1-r && y < y0+r) {
                        int dx = x-(x1-r), dy = y0+r-y;
                        if (dx*dx+dy*dy > r*r) inside = 0;
                    } else if (x < x0+r && y > y1-r) {
                        int dx = x0+r-x, dy = y-(y1-r);
                        if (dx*dx+dy*dy > r*r) inside = 0;
                    } else if (x > x1-r && y > y1-r) {
                        int dx = x-(x1-r), dy = y-(y1-r);
                        if (dx*dx+dy*dy > r*r) inside = 0;
                    }
                }
            }
            if (inside) {
                int on_border = (x==x0 || x==x1 || y==y0 || y==y1);
                int on_corner = 0;
                if (r > 0) {
                    if (x < x0+r && y < y0+r) {
                        int dx=x0+r-x, dy=y0+r-y, d2=dx*dx+dy*dy;
                        on_corner = (d2 >= (r-1)*(r-1) && d2 <= r*r);
                    } else if (x > x1-r && y < y0+r) {
                        int dx=x-(x1-r), dy=y0+r-y, d2=dx*dx+dy*dy;
                        on_corner = (d2 >= (r-1)*(r-1) && d2 <= r*r);
                    } else if (x < x0+r && y > y1-r) {
                        int dx=x0+r-x, dy=y-(y1-r), d2=dx*dx+dy*dy;
                        on_corner = (d2 >= (r-1)*(r-1) && d2 <= r*r);
                    } else if (x > x1-r && y > y1-r) {
                        int dx=x-(x1-r), dy=y-(y1-r), d2=dx*dx+dy*dy;
                        on_corner = (d2 >= (r-1)*(r-1) && d2 <= r*r);
                    }
                }
                put_pixel(buf, w, x, y, (on_border||on_corner) ? stroke : fill);
            }
        }
    }
}

static int lerp_int(int a, int b, float t) { return (int)(a + (b - a) * t); }

/* ── Component renderers ── */

static void render_badge(uint8_t* buf, int w, RGBA bg, RGBA fg, RGBA accent) {
    fill_rounded_rect(buf, w, 10, 10, 100, 34, 12, accent);
    draw_number(buf, w, 20, 19, 128, 1, WHITE);
    draw_string(buf, w, 52, 19, "INBOX", 5, 1, WHITE);
}

static void render_button(uint8_t* buf, int w, RGBA theme_bg, RGBA fg, RGBA accent, RGBA border, RGBA red, int variant, int x, int y) {
    const char* labels[] = {"CLICK","SAVE","DELETE"};
    RGBA fills[] = {theme_bg, accent, red};
    RGBA strokes[] = {border, accent, red};
    RGBA text_cols[] = {fg, WHITE, WHITE};
    int v = variant < 3 ? variant : 0;
    draw_bordered_rounded_rect(buf, w, x, y, x+100, y+28, 6, fills[v], strokes[v]);
    draw_string(buf, w, x+15, y+10, labels[v], 5, 1, text_cols[v]);
}

static void render_checkbox(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA accent, int state, int x, int y) {
    int sz = 18;
    RGBA bc = (state == 0) ? border : accent;
    fill_rounded_rect(buf, w, x, y, x+sz, y+sz, 4, bg);
    fill_hline(buf, w, x+1, x+sz-1, y, bc);
    fill_hline(buf, w, x+1, x+sz-1, y+sz, bc);
    fill_vline(buf, w, x, y+1, y+sz-1, bc);
    fill_vline(buf, w, x+sz, y+1, y+sz-1, bc);
    if (state == 1) {
        fill_rounded_rect(buf, w, x+1, y+1, x+sz-1, y+sz-1, 3, accent);
        /* checkmark pixels */
        put_pixel(buf, w, x+5, y+9, WHITE);
        put_pixel(buf, w, x+6, y+10, WHITE);
        put_pixel(buf, w, x+7, y+11, WHITE);
        put_pixel(buf, w, x+8, y+10, WHITE);
        put_pixel(buf, w, x+9, y+9, WHITE);
        put_pixel(buf, w, x+10, y+8, WHITE);
        put_pixel(buf, w, x+11, y+7, WHITE);
        put_pixel(buf, w, x+12, y+6, WHITE);
        put_pixel(buf, w, x+13, y+5, WHITE);
    } else if (state == 2) {
        fill_hline(buf, w, x+4, x+sz-4, y+sz/2, WHITE);
    }
}

static void render_switch(uint8_t* buf, int w, RGBA accent, RGBA border, int on, int x, int y) {
    int tw = 42, th = 24, tr = 10;
    RGBA tc = on ? accent : border;
    fill_rounded_rect(buf, w, x, y, x+tw, y+th, 12, tc);
    int thumb_x = on ? x+tw-tr-4 : x+tr+4;
    fill_circle(buf, w, thumb_x, y+th/2, tr, WHITE);
}

static void render_progress(uint8_t* buf, int w, RGBA bg2, RGBA accent, int determinate, int64_t now_ms, int x, int y) {
    int pw = 200, ph = 8;
    fill_rounded_rect(buf, w, x, y, x+pw, y+ph, 4, bg2);
    if (determinate) {
        int fw = pw * 60 / 100;
        fill_rounded_rect(buf, w, x, y, x+fw, y+ph, 4, accent);
    } else {
        int offset = (int)((now_ms / 50) % pw);
        for (int sx = -pw; sx < pw*2; sx += 20) {
            int ax = x + sx + offset;
            if (ax+10 > x && ax < x+pw) {
                int cx = ax < x ? x : ax;
                int ce = ax+10 > x+pw ? x+pw : ax+10;
                fill_rounded_rect(buf, w, cx, y, ce, y+ph, 2, accent);
            }
        }
    }
}

static void render_slider(uint8_t* buf, int w, RGBA bg2, RGBA accent, float value, int x, int y) {
    int sw = 200, sh = 6, tr = 8;
    fill_rounded_rect(buf, w, x, y+tr-sh/2, x+sw, y+tr+sh/2, 3, bg2);
    int filled = (int)(sw * value);
    fill_rounded_rect(buf, w, x, y+tr-sh/2, x+filled, y+tr+sh/2, 3, accent);
    fill_circle(buf, w, x+filled, y+tr, tr, accent);
    fill_circle(buf, w, x+filled, y+tr, tr-2, WHITE);
}

static void render_spinner(uint8_t* buf, int w, RGBA fg, int64_t now_ms, int cx, int cy) {
    int radius = 12;
    int phase = (int)(now_ms / 100);
    for (int i = 0; i < 8; i++) {
        float angle = ((float)i + (float)(phase % 8)) / 8.0f * 3.14159f * 2.0f;
        float alpha_f = 1.0f - (float)i / 8.0f;
        uint8_t alpha = (uint8_t)(alpha_f * 255.0f);
        RGBA ac = color_alpha(fg, alpha);
        float cos_a = 0, sin_a = 0;
        /* simple sin/cos approximation */
        float a2 = angle - 6.28318f * (int)(angle / 6.28318f);
        if (a2 < 0) a2 += 6.28318f;
        /* use integer approximation for sin/cos */
        int ir = radius - 3, or = radius;
        int x1 = cx + (int)(ir * 0.707f * ((a2 < 1.57f) ? 1 : (a2 < 3.14f) ? -1 : (a2 < 4.71f) ? -1 : 1));
        int y1 = cy + (int)(ir * 0.707f * ((a2 < 3.14f) ? 1 : -1));
        /* simplified: just draw radial lines */
        int steps = 4;
        for (int s = 0; s <= steps; s++) {
            float t = (float)s / (float)steps;
            int px = lerp_int(cx, cx + (int)(or * 0.7f), t);
            int py = lerp_int(cy, cy + (int)(or * 0.7f), t);
            /* rotate based on angle */
            float c = 1.0f, sn = 0.0f;
            /* very rough rotation */
            int rx = cx + (int)((px-cx)*c - (py-cy)*sn);
            int ry = cy + (int)((px-cx)*sn + (py-cy)*c);
            put_pixel(buf, w, rx, ry, ac);
            put_pixel(buf, w, rx+1, ry, ac);
            put_pixel(buf, w, rx, ry+1, ac);
        }
    }
}

static void render_stepper(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, int value, int x, int y) {
    int bw = 28, bh = 24;
    draw_bordered_rounded_rect(buf, w, x, y, x+bw, y+bh, 6, bg, border);
    fill_hline(buf, w, x+9, x+19, y+12, fg);
    draw_bordered_rounded_rect(buf, w, x+bw+2, y, x+bw*2+2, y+bh, 6, bg, border);
    fill_hline(buf, w, x+bw+11, x+bw+21, y+12, fg);
    fill_vline(buf, w, x+bw+16, y+7, y+17, fg);
    const char* vals[] = {"0","1","2","3","4","5","6","7","8","9"};
    int v = value < 0 ? 0 : (value > 9 ? 9 : value);
    draw_string(buf, w, x+bw+8, y+9, vals[v], 1, 1, fg);
}

static void render_separator(uint8_t* buf, int w, RGBA border) {
    fill_hline(buf, w, 10, LOGICAL_W-10, 470, border);
}

static void render_label(uint8_t* buf, int w, RGBA fg, const char* text, int len, int x, int y, int scale) {
    draw_string(buf, w, x, y, text, len, scale, fg);
}

static void render_glass_panel(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, int x, int y) {
    int pw = 260, ph = 50;
    uint8_t bg_a = (bg.r == 255) ? 180 : 160;
    uint8_t brd_a = (bg.r == 255) ? 100 : 80;
    fill_rounded_rect(buf, w, x, y, x+pw, y+ph, 10, color_alpha(bg, bg_a));
    fill_hline(buf, w, x+10, x+pw-10, y, color_alpha(border, brd_a));
    fill_hline(buf, w, x+10, x+pw-10, y+ph, color_alpha(border, brd_a));
    fill_vline(buf, w, x, y+10, y+ph-10, color_alpha(border, brd_a));
    fill_vline(buf, w, x+pw, y+10, y+ph-10, color_alpha(border, brd_a));
    draw_string(buf, w, x+10, y+10, "GLASS PANEL", 11, 1, fg);
}

static void render_help_button(uint8_t* buf, int w, RGBA accent, int x, int y) {
    int r = 12;
    fill_circle(buf, w, x+r, y+r, r, accent);
    put_pixel(buf, w, x+r-2, y+r-4, WHITE);
    put_pixel(buf, w, x+r-1, y+r-5, WHITE);
    put_pixel(buf, w, x+r, y+r-5, WHITE);
    put_pixel(buf, w, x+r+1, y+r-5, WHITE);
    put_pixel(buf, w, x+r+2, y+r-4, WHITE);
    put_pixel(buf, w, x+r+2, y+r-3, WHITE);
    put_pixel(buf, w, x+r+1, y+r-2, WHITE);
    put_pixel(buf, w, x+r, y+r-1, WHITE);
    put_pixel(buf, w, x+r, y+r+2, WHITE);
}

static void render_radio_group(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA accent, RGBA fg, uint8_t selected, int x, int y) {
    const char* labels[] = {"ALPHA","BETA","GAMMA"};
    int lens[] = {5,4,4};
    for (int i = 0; i < 3; i++) {
        int ry = y + i * 24;
        int rr = 8, cx = x + rr;
        fill_circle(buf, w, cx, ry+rr, rr, border);
        fill_circle(buf, w, cx, ry+rr, rr-2, bg);
        if (i == selected) fill_circle(buf, w, cx, ry+rr, rr-3, accent);
        draw_string(buf, w, x+24, ry+rr-3, labels[i], lens[i], 1, fg);
    }
}

static void render_search_field(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA accent, uint32_t search_len, int x, int y) {
    int fw = 240, fh = 28;
    draw_bordered_rounded_rect(buf, w, x, y, x+fw, y+fh, 14, bg, border);
    int icx = x+16, icy = y+14;
    fill_circle(buf, w, icx, icy, 5, border);
    fill_circle(buf, w, icx, icy, 3, bg);
    put_pixel(buf, w, icx+4, icy+4, border);
    put_pixel(buf, w, icx+5, icy+5, border);
    put_pixel(buf, w, icx+6, icy+6, border);
    if (search_len > 0) {
        fill_vline(buf, w, x+30, y+8, y+20, accent);
    } else {
        draw_string(buf, w, x+30, y+9, "SEARCH", 6, 1, border);
    }
}

static void render_secure_field(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, uint32_t content_len, int x, int y) {
    int fw = 240, fh = 28;
    draw_bordered_rounded_rect(buf, w, x, y, x+fw, y+fh, 6, bg, border);
    for (uint32_t i = 0; i < content_len && i < 10; i++)
        fill_circle(buf, w, x+12+(int)i*12, y+14, 3, fg);
    if (content_len == 0)
        draw_string(buf, w, x+10, y+9, "PASSWORD", 8, 1, border);
}

static void render_text_field(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA accent, uint32_t content_len, int x, int y) {
    int fw = 240, fh = 28;
    draw_bordered_rounded_rect(buf, w, x, y, x+fw, y+fh, 6, bg, border);
    if (content_len > 0)
        fill_vline(buf, w, x+10+(int)content_len*6, y+8, y+20, accent);
    else
        draw_string(buf, w, x+10, y+9, "PLACEHOLDER", 11, 1, border);
}

static void render_select(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, RGBA accent, uint8_t index, uint32_t is_open, int x, int y) {
    int fw = 240, fh = 28;
    const char* labels[] = {"OPTION A","OPTION B","OPTION C"};
    int lens[] = {8,8,8};
    draw_bordered_rounded_rect(buf, w, x, y, x+fw, y+fh, 6, bg, border);
    int idx = index < 3 ? index : 0;
    draw_string(buf, w, x+10, y+9, labels[idx], lens[idx], 1, fg);
    put_pixel(buf, w, x+fw-16, y+12, fg);
    put_pixel(buf, w, x+fw-15, y+13, fg);
    put_pixel(buf, w, x+fw-14, y+14, fg);
    put_pixel(buf, w, x+fw-13, y+13, fg);
    put_pixel(buf, w, x+fw-12, y+12, fg);
    if (is_open) {
        for (int i = 0; i < 3; i++) {
            int my = y + fh + i * (fh-4);
            fill_rounded_rect(buf, w, x, my, x+fw, my+fh-4, 0, bg);
            RGBA tc = (i == idx) ? accent : fg;
            draw_string(buf, w, x+10, my+9, labels[i], lens[i], 1, tc);
        }
    }
}

static void render_popup_button(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, uint8_t index, uint32_t is_open, int x, int y) {
    int bw = 160, bh = 28;
    const char* labels[] = {"FILE","EDIT","VIEW"};
    int lens[] = {4,4,4};
    draw_bordered_rounded_rect(buf, w, x, y, x+bw, y+bh, 6, bg, border);
    int idx = index < 3 ? index : 0;
    draw_string(buf, w, x+10, y+9, labels[idx], lens[idx], 1, fg);
    put_pixel(buf, w, x+bw-16, y+12, fg);
    put_pixel(buf, w, x+bw-15, y+13, fg);
    put_pixel(buf, w, x+bw-14, y+14, fg);
    put_pixel(buf, w, x+bw-13, y+13, fg);
    put_pixel(buf, w, x+bw-12, y+12, fg);
    if (is_open) {
        for (int i = 0; i < 3; i++) {
            int my = y + bh + i * (bh-4);
            fill_rounded_rect(buf, w, x, my, x+bw, my+bh-4, 0, bg);
            draw_string(buf, w, x+10, my+9, labels[i], lens[i], 1, fg);
        }
    }
}

static void render_segmented_control(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, uint8_t selected, int x, int y) {
    int sw = 80, sh = 28, tw = sw*3;
    const char* labels[] = {"FIRST","SECOND","THIRD"};
    int lens[] = {5,6,5};
    draw_bordered_rounded_rect(buf, w, x, y, x+tw, y+sh, 6, bg, border);
    for (int i = 0; i < 3; i++) {
        int sx = x + i * sw;
        if (i == selected) {
            fill_rounded_rect(buf, w, sx+1, y+1, sx+sw-1, y+sh-1, 5, fg);
            draw_string(buf, w, sx+10, y+9, labels[i], lens[i], 1, WHITE);
        } else {
            draw_string(buf, w, sx+10, y+9, labels[i], lens[i], 1, fg);
        }
        if (i > 0) fill_vline(buf, w, sx, y+4, y+sh-4, border);
    }
}

static void render_box(uint8_t* buf, int w, RGBA bg, RGBA border, RGBA fg, int x, int y) {
    draw_bordered_rounded_rect(buf, w, x, y, x+260, y+40, 6, bg, border);
    draw_string(buf, w, x+10, y+10, "GROUPED BOX", 11, 1, fg);
}

static void render_panel(uint8_t* buf, int w, RGBA bg2, RGBA fg, int x, int y) {
    fill_rounded_rect(buf, w, x, y, x+260, y+40, 6, bg2);
    draw_string(buf, w, x+10, y+10, "SURFACE PANEL", 13, 1, fg);
}

/* ── Overview renderer ── */

static void render_overview(uint8_t* buf, Instance* inst) {
    RGBA bg = inst->dark ? DARK_BG : LIGHT_BG;
    RGBA fg = inst->dark ? DARK_FG : LIGHT_FG;
    RGBA accent = inst->dark ? DARK_ACCENT : LIGHT_ACCENT;
    RGBA border = inst->dark ? DARK_BORDER : LIGHT_BORDER;
    RGBA bg2 = inst->dark ? DARK_BG2 : LIGHT_BG2;
    int sy = (int)inst->scroll_y;
    int y0;

    /* Badge */
    render_badge(buf, LOGICAL_W, bg, fg, accent);
    y0 = 50;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "BADGE", 5, 1, border);

    /* Buttons */
    y0 = 70;
    render_button(buf, LOGICAL_W, bg, fg, accent, border, DARK_RED, 0, 10, y0-sy);
    render_button(buf, LOGICAL_W, bg, fg, accent, border, DARK_RED, 1, 120, y0-sy);
    render_button(buf, LOGICAL_W, bg, fg, accent, border, DARK_RED, 2, 230, y0-sy);
    y0 = 110;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "BUTTONS", 7, 1, border);

    /* Checkboxes */
    y0 = 130;
    render_checkbox(buf, LOGICAL_W, bg, border, accent, 0, 10, y0-sy);
    draw_string(buf, LOGICAL_W, 36, y0-sy+1, "UNCHECKED", 9, 1, fg);
    render_checkbox(buf, LOGICAL_W, bg, border, accent, 1, 110, y0-sy);
    draw_string(buf, LOGICAL_W, 136, y0-sy+1, "CHECKED", 7, 1, fg);
    render_checkbox(buf, LOGICAL_W, bg, border, accent, 2, 210, y0-sy);
    draw_string(buf, LOGICAL_W, 236, y0-sy+1, "INDET", 5, 1, fg);
    y0 = 160;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "CHECKBOXES", 10, 1, border);

    /* Switches */
    y0 = 180;
    render_switch(buf, LOGICAL_W, accent, border, 0, 10, y0-sy);
    draw_string(buf, LOGICAL_W, 62, y0-sy+3, "OFF", 3, 1, fg);
    render_switch(buf, LOGICAL_W, accent, border, 1, 110, y0-sy);
    draw_string(buf, LOGICAL_W, 162, y0-sy+3, "ON", 2, 1, fg);
    y0 = 220;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SWITCHES", 8, 1, border);

    /* Progress */
    y0 = 240;
    render_progress(buf, LOGICAL_W, bg2, accent, 1, inst->now_ms, 10, y0-sy);
    render_progress(buf, LOGICAL_W, bg2, accent, 0, inst->now_ms, 10, y0-sy+20);
    y0 = 280;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "PROGRESS", 8, 1, border);

    /* Slider */
    y0 = 300;
    render_slider(buf, LOGICAL_W, bg2, accent, inst->slider_value, 10, y0-sy);
    y0 = 330;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SLIDER", 6, 1, border);

    /* Spinner */
    y0 = 350;
    render_spinner(buf, LOGICAL_W, fg, inst->now_ms, 22, y0-sy+12);
    draw_string(buf, LOGICAL_W, 40, y0-sy+6, "SPINNER", 7, 1, fg);
    y0 = 380;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SPINNER", 7, 1, border);

    /* Stepper */
    y0 = 400;
    render_stepper(buf, LOGICAL_W, bg, border, fg, inst->stepper_value, 10, y0-sy);
    y0 = 440;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "STEPPER", 7, 1, border);

    /* Separator */
    y0 = 460;
    fill_hline(buf, LOGICAL_W, 10, LOGICAL_W-10, y0-sy, border);

    /* Labels */
    y0 = 480;
    render_label(buf, LOGICAL_W, fg, "LARGE TITLE", 11, 10, y0-sy, 2);
    y0 = 510;
    render_label(buf, LOGICAL_W, fg, "TITLE 1", 7, 10, y0-sy, 2);
    y0 = 540;
    render_label(buf, LOGICAL_W, fg, "BODY TEXT", 9, 10, y0-sy, 1);
    y0 = 555;
    render_label(buf, LOGICAL_W, fg, "CAPTION", 7, 10, y0-sy, 1);
    y0 = 575;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "LABELS", 6, 1, border);

    /* Glass panel */
    y0 = 595;
    render_glass_panel(buf, LOGICAL_W, bg, border, fg, 10, y0-sy);
    y0 = 660;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "GLASS PANEL", 11, 1, border);

    /* Help button */
    y0 = 680;
    render_help_button(buf, LOGICAL_W, accent, 22, y0-sy+12);
    draw_string(buf, LOGICAL_W, 50, y0-sy+6, "HELP BUTTON", 11, 1, fg);
    y0 = 710;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "HELP", 4, 1, border);

    /* Radio group */
    y0 = 730;
    render_radio_group(buf, LOGICAL_W, bg, border, accent, fg, inst->radio_selected, 10, y0-sy);
    y0 = 810;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "RADIO GROUP", 11, 1, border);

    /* Search field */
    y0 = 830;
    render_search_field(buf, LOGICAL_W, bg, border, accent, inst->search_len, 10, y0-sy);
    y0 = 870;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SEARCH FIELD", 12, 1, border);

    /* Secure field */
    y0 = 890;
    render_secure_field(buf, LOGICAL_W, bg, border, fg, inst->secure_field_len, 10, y0-sy);
    y0 = 930;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SECURE FIELD", 12, 1, border);

    /* Text field */
    y0 = 950;
    render_text_field(buf, LOGICAL_W, bg, border, accent, inst->text_field_len, 10, y0-sy);
    y0 = 990;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "TEXT FIELD", 10, 1, border);

    /* Select */
    y0 = 1010;
    render_select(buf, LOGICAL_W, bg, border, fg, accent, inst->select_index, inst->select_open, 10, y0-sy);
    y0 = 1050;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SELECT", 6, 1, border);

    /* Popup button */
    y0 = 1070;
    render_popup_button(buf, LOGICAL_W, bg, border, fg, inst->popup_index, inst->popup_open, 10, y0-sy);
    y0 = 1110;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "POPUP BUTTON", 12, 1, border);

    /* Segmented control */
    y0 = 1130;
    render_segmented_control(buf, LOGICAL_W, bg, border, fg, inst->segmented_index, 10, y0-sy);
    y0 = 1170;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "SEGMENTED", 9, 1, border);

    /* Box */
    y0 = 1190;
    render_box(buf, LOGICAL_W, bg, border, fg, 10, y0-sy);
    y0 = 1240;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "BOX", 3, 1, border);

    /* Panel */
    y0 = 1260;
    render_panel(buf, LOGICAL_W, bg2, fg, 10, y0-sy);
    y0 = 1310;
    draw_string(buf, LOGICAL_W, 10, y0-sy, "PANEL", 5, 1, border);
}

/* ── WASM exports ── */

WASM_EXPORT uint32_t preview_alloc(uint32_t len) {
    uint32_t ptr = bump_alloc(len);
    if (ptr) render_buf_ptr = ptr;
    return ptr;
}

WASM_EXPORT void preview_free(uint32_t ptr) { (void)ptr; }

WASM_EXPORT uint32_t preview_create(uint32_t name_ptr, uint32_t name_len, uint32_t dark) {
    if (instance_count >= MAX_INSTANCES) return 0;
    Instance* inst = &instances[instance_count];
    uint8_t* mem = wasm_mem();
    memset(inst, 0, sizeof(Instance));
    uint32_t to_copy = name_len < 63 ? name_len : 63;
    memcpy(inst->name, mem + name_ptr, to_copy);
    inst->name_len = to_copy;
    inst->dark = dark;
    inst->checkbox_checked = 1;
    inst->switch_on = 1;
    inst->progress_value = 0.6f;
    inst->slider_value = 0.4f;
    instance_count++;
    return instance_count;
}

WASM_EXPORT void preview_render(uint32_t preview_id, uint32_t scale) {
    (void)scale;
    if (preview_id == 0 || preview_id > instance_count) return;
    Instance* inst = &instances[preview_id - 1];
    if (render_buf_ptr == 0) return;
    uint8_t* mem = wasm_mem();
    uint8_t* buf = mem + render_buf_ptr;
    RGBA bg = inst->dark ? DARK_BG : LIGHT_BG;
    fill_rounded_rect(buf, LOGICAL_W, 0, 0, LOGICAL_W-1, LOGICAL_H-1, 0, bg);
    render_overview(buf, inst);
}

WASM_EXPORT void preview_pointer(uint32_t preview_id, uint32_t kind, float x, float y) {
    if (preview_id == 0 || preview_id > instance_count) return;
    Instance* inst = &instances[preview_id - 1];
    inst->ptr_x = x; inst->ptr_y = y;
    int ix = (int)x, iy = (int)y + (int)inst->scroll_y;
    if (kind == 0) {
        inst->ptr_down = 1;
        if (iy >= 130 && iy <= 148) {
            if (ix >= 10 && ix <= 28) { inst->checkbox_checked = 0; inst->checkbox_indeterminate = 0; }
            else if (ix >= 110 && ix <= 128) { inst->checkbox_checked = 1; inst->checkbox_indeterminate = 0; }
            else if (ix >= 210 && ix <= 228) { inst->checkbox_checked = 0; inst->checkbox_indeterminate = 1; }
        }
        if (iy >= 180 && iy <= 204 && ix >= 10 && ix <= 52) inst->switch_on = !inst->switch_on;
        if (iy >= 730 && iy <= 800) {
            int ry = iy - 730;
            if (ry < 24) inst->radio_selected = 0;
            else if (ry < 48) inst->radio_selected = 1;
            else if (ry < 72) inst->radio_selected = 2;
        }
        if (iy >= 1130 && iy <= 1158) {
            int rx = ix - 10;
            if (rx < 80) inst->segmented_index = 0;
            else if (rx < 160) inst->segmented_index = 1;
            else if (rx < 240) inst->segmented_index = 2;
        }
        if (iy >= 1010 && iy <= 1038 && ix >= 10 && ix <= 250) inst->select_open = !inst->select_open;
        else if (inst->select_open && iy >= 1038 && iy <= 1090) {
            int ry = iy - 1038;
            if (ry < 24) inst->select_index = 0;
            else if (ry < 48) inst->select_index = 1;
            else if (ry < 72) inst->select_index = 2;
            inst->select_open = 0;
        }
        if (iy >= 1070 && iy <= 1098 && ix >= 10 && ix <= 170) inst->popup_open = !inst->popup_open;
        else if (inst->popup_open && iy >= 1098 && iy <= 1150) {
            int ry = iy - 1098;
            if (ry < 24) inst->popup_index = 0;
            else if (ry < 48) inst->popup_index = 1;
            else if (ry < 72) inst->popup_index = 2;
            inst->popup_open = 0;
        }
        if (iy >= 400 && iy <= 424) {
            if (ix >= 10 && ix <= 38) inst->stepper_value--;
            else if (ix >= 40 && ix <= 68) inst->stepper_value++;
        }
    } else if (kind == 1) {
        inst->ptr_down = 0;
    }
}

WASM_EXPORT void preview_scroll(uint32_t preview_id, float dx, float dy) {
    if (preview_id == 0 || preview_id > instance_count) return;
    Instance* inst = &instances[preview_id - 1];
    inst->scroll_y += dy;
    if (inst->scroll_y < 0) inst->scroll_y = 0;
    if (inst->scroll_y > 1200) inst->scroll_y = 1200;
}

WASM_EXPORT void preview_key(uint32_t preview_id, uint32_t key_ptr, uint32_t key_len, uint32_t modifiers) {
    (void)modifiers;
    if (preview_id == 0 || preview_id > instance_count) return;
    Instance* inst = &instances[preview_id - 1];
    uint8_t* mem = wasm_mem();
    if (key_len == 1) {
        uint8_t ch = mem[key_ptr];
        if (ch == '\t') inst->segmented_index = (inst->segmented_index + 1) % 3;
        else if (ch == ' ') inst->switch_on = !inst->switch_on;
    }
}

WASM_EXPORT void preview_text(uint32_t preview_id, uint32_t text_ptr, uint32_t text_len) {
    if (preview_id == 0 || preview_id > instance_count) return;
    Instance* inst = &instances[preview_id - 1];
    uint8_t* mem = wasm_mem();
    for (uint32_t i = 0; i < text_len; i++) {
        if (inst->text_field_len < 63)
            inst->text_field_buf[inst->text_field_len++] = mem[text_ptr + i];
    }
}

WASM_EXPORT void preview_destroy(uint32_t preview_id) { (void)preview_id; }

WASM_EXPORT void preview_set_now_ms(uint32_t preview_id, uint32_t ms) {
    if (preview_id == 0 || preview_id > instance_count) return;
    instances[preview_id - 1].now_ms = (int64_t)ms;
}

WASM_EXPORT void preview_set_theme(uint32_t preview_id, uint32_t dark) {
    if (preview_id == 0 || preview_id > instance_count) return;
    instances[preview_id - 1].dark = dark;
}

WASM_EXPORT uint32_t preview_logical_width(uint32_t preview_id) {
    if (preview_id == 0 || preview_id > instance_count) return 0;
    return LOGICAL_W;
}

WASM_EXPORT uint32_t preview_logical_height(uint32_t preview_id) {
    if (preview_id == 0 || preview_id > instance_count) return 0;
    return LOGICAL_H;
}

WASM_EXPORT uint32_t preview_cursor(uint32_t preview_id) {
    if (preview_id == 0 || preview_id > instance_count) return 0;
    return instances[preview_id - 1].cursor;
}

WASM_EXPORT uint32_t preview_pixel_byte_len(uint32_t preview_id) {
    if (preview_id == 0 || preview_id > instance_count) return 0;
    return LOGICAL_W * LOGICAL_H * 4;
}

WASM_EXPORT uint32_t preview_status(uint32_t preview_id) {
    if (preview_id == 0 || preview_id > instance_count) return 0;
    return 1;
}
