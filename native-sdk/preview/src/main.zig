const std = @import("std");

// ── WASM linear memory ────────────────────────────────────────────────
const PAGE_SIZE: usize = 65536;
const INITIAL_PAGES: u32 = 256;
export var memory: [1]u8 = undefined;

var mem_base: usize = 0;
var mem_len: usize = 0;

// ── Bump allocator ────────────────────────────────────────────────────
var bump_ptr: usize = 0;

fn bumpAlloc(len: usize) ?usize {
    const aligned = (bump_ptr + 7) & ~@as(usize, 7);
    if (aligned + len > mem_len) return null;
    const result = aligned;
    bump_ptr = aligned + len;
    return result;
}

// ── Colours ───────────────────────────────────────────────────────────
const RGBA = [4]u8;

const Theme = struct {
    bg: RGBA,
    fg: RGBA,
    accent: RGBA,
    border: RGBA,
    bg_secondary: RGBA,
    green: RGBA,
    red: RGBA,
};

const LIGHT = Theme{
    .bg = .{ 255, 255, 255, 255 },
    .fg = .{ 0, 0, 0, 255 },
    .accent = .{ 0, 122, 255, 255 },
    .border = .{ 216, 216, 220, 255 },
    .bg_secondary = .{ 240, 240, 245, 255 },
    .green = .{ 52, 199, 89, 255 },
    .red = .{ 255, 59, 48, 255 },
};

const DARK = Theme{
    .bg = .{ 30, 30, 30, 255 },
    .fg = .{ 255, 255, 255, 255 },
    .accent = .{ 10, 132, 255, 255 },
    .border = .{ 55, 55, 55, 255 },
    .bg_secondary = .{ 44, 44, 46, 255 },
    .green = .{ 48, 209, 88, 255 },
    .red = .{ 255, 69, 58, 255 },
};

// ── Cursor constants ──────────────────────────────────────────────────
const CURSOR_DEFAULT: u32 = 0;
const CURSOR_POINTER: u32 = 1;
const CURSOR_TEXT: u32 = 2;
const CURSOR_COL_RESIZE: u32 = 3;

// ── Pointer event kinds ───────────────────────────────────────────────
const PTR_DOWN: u32 = 0;
const PTR_UP: u32 = 1;
const PTR_MOVE: u32 = 2;

// ── Logical component sizes ───────────────────────────────────────────
const LOGICAL_W: i32 = 320;
const LOGICAL_H: i32 = 200;

// ── Pixel buffer tracking ─────────────────────────────────────────────
var render_buf_ptr: usize = 0;

// ── Preview instance ──────────────────────────────────────────────────
const Instance = struct {
    name: []u8,
    dark: bool,
    now_ms: i64,
    cursor: u32,
    scroll_x: f32,
    scroll_y: f32,
    ptr_x: f32,
    ptr_y: f32,
    ptr_down: bool,
    checkbox_checked: bool,
    checkbox_indeterminate: bool,
    switch_on: bool,
    progress_value: f32,
    slider_value: f32,
    radio_selected: u8,
    text_field_buf: [64]u8,
    text_field_len: usize,
    secure_field_buf: [64]u8,
    secure_field_len: usize,
    select_open: bool,
    select_index: u8,
    popup_open: bool,
    popup_index: u8,
    segmented_index: u8,
    stepper_value: i32,
    search_buf: [64]u8,
    search_len: usize,
};

var instances: [16]Instance = undefined;
var instance_count: usize = 0;

// ── Pixel helpers ─────────────────────────────────────────────────────

fn putPixel(buf: []u8, w: usize, x: i32, y: i32, c: RGBA) void {
    if (x < 0 or y < 0) return;
    const ux: usize = @intCast(x);
    const uy: usize = @intCast(y);
    if (ux >= w or uy >= buf.len / (w * 4)) return;
    const off = (uy * w + ux) * 4;
    if (off + 3 >= buf.len) return;
    buf[off] = c[0];
    buf[off + 1] = c[1];
    buf[off + 2] = c[2];
    buf[off + 3] = c[3];
}

fn blendPixel(buf: []u8, w: usize, x: i32, y: i32, c: RGBA) void {
    if (x < 0 or y < 0) return;
    const ux: usize = @intCast(x);
    const uy: usize = @intCast(y);
    if (ux >= w or uy >= buf.len / (w * 4)) return;
    const off = (uy * w + ux) * 4;
    if (off + 3 >= buf.len) return;
    const a: u32 = c[3];
    const inv: u32 = 255 - a;
    buf[off] = @intCast((@as(u32, buf[off]) * inv + @as(u32, c[0]) * a) / 255);
    buf[off + 1] = @intCast((@as(u32, buf[off + 1]) * inv + @as(u32, c[1]) * a) / 255);
    buf[off + 2] = @intCast((@as(u32, buf[off + 2]) * inv + @as(u32, c[2]) * a) / 255);
    buf[off + 3] = 255;
}

fn fillRect(buf: []u8, w: usize, x0: i32, y0: i32, x1: i32, y1: i32, c: RGBA) void {
    var y = y0;
    while (y <= y1) : (y += 1) {
        var x = x0;
        while (x <= x1) : (x += 1) {
            putPixel(buf, w, x, y, c);
        }
    }
}

fn fillRoundedRect(buf: []u8, w: usize, bx0: i32, by0: i32, bx1: i32, by1: i32, r: i32, c: RGBA) void {
    var y = by0;
    while (y <= by1) : (y += 1) {
        var x = bx0;
        while (x <= bx1) : (x += 1) {
            var inside = true;
            if (x < bx0 or x > bx1 or y < by0 or y > by1) {
                inside = false;
            } else if (x < bx0 + r and y < by0 + r) {
                const dx = @as(i32, bx0 + r) - x;
                const dy = @as(i32, by0 + r) - y;
                if (dx * dx + dy * dy > r * r) inside = false;
            } else if (x > bx1 - r and y < by0 + r) {
                const dx = x - @as(i32, bx1 - r);
                const dy = @as(i32, by0 + r) - y;
                if (dx * dx + dy * dy > r * r) inside = false;
            } else if (x < bx0 + r and y > by1 - r) {
                const dx = @as(i32, bx0 + r) - x;
                const dy = y - @as(i32, by1 - r);
                if (dx * dx + dy * dy > r * r) inside = false;
            } else if (x > bx1 - r and y > by1 - r) {
                const dx = x - @as(i32, bx1 - r);
                const dy = y - @as(i32, by1 - r);
                if (dx * dx + dy * dy > r * r) inside = false;
            }
            if (inside) putPixel(buf, w, x, y, c);
        }
    }
}

fn fillCircle(buf: []u8, w: usize, cx: i32, cy: i32, radius: i32, c: RGBA) void {
    var y = cy - radius;
    while (y <= cy + radius) : (y += 1) {
        var x = cx - radius;
        while (x <= cx + radius) : (x += 1) {
            const dx = x - cx;
            const dy = y - cy;
            if (dx * dx + dy * dy <= radius * radius) {
                putPixel(buf, w, x, y, c);
            }
        }
    }
}

fn fillHLine(buf: []u8, w: usize, x0: i32, x1: i32, y: i32, c: RGBA) void {
    var x = x0;
    while (x <= x1) : (x += 1) {
        putPixel(buf, w, x, y, c);
    }
}

fn fillVLine(buf: []u8, w: usize, x: i32, y0: i32, y1: i32, c: RGBA) void {
    var y = y0;
    while (y <= y1) : (y += 1) {
        putPixel(buf, w, x, y, c);
    }
}

fn lerp(a: i32, b: i32, t: f32) i32 {
    return @intFromFloat(@as(f32, @floatFromInt(a)) + (@as(f32, @floatFromInt(b)) - @as(f32, @floatFromInt(a))) * t);
}

fn colorAlpha(c: RGBA, a: u8) RGBA {
    return .{ c[0], c[1], c[2], a };
}

// ── Bitmap font (5x6) ────────────────────────────────────────────────
const GLYPH_W = 5;
const GLYPH_H = 6;
const GLYPH_SPACING = 1;

const DIGIT_GLYPHS = [10][6]u8{
    .{ 0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b01110 },
    .{ 0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110 },
    .{ 0b01110, 0b10001, 0b00010, 0b00100, 0b01000, 0b11111 },
    .{ 0b01110, 0b10001, 0b00110, 0b00001, 0b10001, 0b01110 },
    .{ 0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010 },
    .{ 0b11111, 0b10000, 0b11110, 0b00001, 0b10001, 0b01110 },
    .{ 0b00110, 0b01000, 0b11110, 0b10001, 0b10001, 0b01110 },
    .{ 0b11111, 0b00001, 0b00010, 0b00100, 0b00100, 0b00100 },
    .{ 0b01110, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110 },
    .{ 0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110 },
};

const LETTER_GLYPHS = [26][6]u8{
    .{ 0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001 },
    .{ 0b11110, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110 },
    .{ 0b01110, 0b10001, 0b10000, 0b10000, 0b10001, 0b01110 },
    .{ 0b11100, 0b10010, 0b10001, 0b10001, 0b10010, 0b11100 },
    .{ 0b11111, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111 },
    .{ 0b11111, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000 },
    .{ 0b01110, 0b10001, 0b10000, 0b10011, 0b10001, 0b01110 },
    .{ 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001 },
    .{ 0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110 },
    .{ 0b00001, 0b00001, 0b00001, 0b00001, 0b10001, 0b01110 },
    .{ 0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010 },
    .{ 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111 },
    .{ 0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001 },
    .{ 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001 },
    .{ 0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110 },
    .{ 0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000 },
    .{ 0b01110, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101 },
    .{ 0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010 },
    .{ 0b01110, 0b10001, 0b11100, 0b00010, 0b10001, 0b01110 },
    .{ 0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100 },
    .{ 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110 },
    .{ 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100 },
    .{ 0b10001, 0b10001, 0b10001, 0b10101, 0b11011, 0b10001 },
    .{ 0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001 },
    .{ 0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100 },
    .{ 0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111 },
};

const SPACE_GLYPH = [6]u8{ 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000 };

fn drawGlyph(buf: []u8, buf_w: usize, gx: i32, gy: i32, glyph: [6]u8, scale: i32, c: RGBA) void {
    var row: i32 = 0;
    while (row < GLYPH_H) : (row += 1) {
        const bits = glyph[@intCast(row)];
        var col: i32 = 0;
        while (col < GLYPH_W) : (col += 1) {
            const mask: u8 = @as(u8, 1) << @intCast(4 - @as(u32, @intCast(col)));
            if (bits & mask != 0) {
                var sy: i32 = 0;
                while (sy < scale) : (sy += 1) {
                    var sx: i32 = 0;
                    while (sx < scale) : (sx += 1) {
                        putPixel(buf, buf_w, gx + col * scale + sx, gy + row * scale + sy, c);
                    }
                }
            }
        }
    }
}

fn drawDigit(buf: []u8, buf_w: usize, x: i32, y: i32, digit: u8, scale: i32, c: RGBA) void {
    if (digit <= 9) {
        drawGlyph(buf, buf_w, x, y, DIGIT_GLYPHS[digit], scale, c);
    }
}

fn drawNumber(buf: []u8, buf_w: usize, x: i32, y: i32, num: u32, scale: i32, c: RGBA) void {
    if (num == 0) {
        drawDigit(buf, buf_w, x, y, 0, scale, c);
        return;
    }
    var tmp = num;
    var count: u32 = 0;
    while (tmp > 0) : (tmp /= 10) {
        count += 1;
    }
    var dx: i32 = @intCast((count - 1) * (GLYPH_W + GLYPH_SPACING) * @as(u32, @intCast(scale)));
    tmp = num;
    while (tmp > 0) : (tmp /= 10) {
        const d: u8 = @intCast(tmp % 10);
        drawDigit(buf, buf_w, x + dx, y, d, scale, c);
        dx -= @as(i32, @intCast((GLYPH_W + GLYPH_SPACING) * @as(u32, @intCast(scale))));
    }
}

fn drawLetter(buf: []u8, buf_w: usize, x: i32, y: i32, ch: u8, scale: i32, c: RGBA) void {
    if (ch == ' ') {
        drawGlyph(buf, buf_w, x, y, SPACE_GLYPH, scale, c);
    } else if (ch >= 'A' and ch <= 'Z') {
        drawGlyph(buf, buf_w, x, y, LETTER_GLYPHS[ch - 'A'], scale, c);
    } else if (ch >= 'a' and ch <= 'z') {
        drawGlyph(buf, buf_w, x, y, LETTER_GLYPHS[ch - 'a'], scale, c);
    } else if (ch >= '0' and ch <= '9') {
        drawDigit(buf, buf_w, x, y, ch - '0', scale, c);
    }
}

fn drawString(buf: []u8, buf_w: usize, x: i32, y: i32, text: []const u8, scale: i32, c: RGBA) void {
    var i: usize = 0;
    var cx = x;
    while (i < text.len) : (i += 1) {
        drawLetter(buf, buf_w, cx, y, text[i], scale, c);
        cx += @as(i32, @intCast((GLYPH_W + GLYPH_SPACING) * @as(u32, @intCast(scale))));
    }
}

// ── Draw bordered rounded rect ────────────────────────────────────────

fn drawBorderedRoundedRect(buf: []u8, buf_w: usize, x0: i32, y0: i32, x1: i32, y1: i32, r: i32, fill: RGBA, stroke: RGBA) void {
    var y: i32 = y0;
    while (y <= y1) : (y += 1) {
        var x: i32 = x0;
        while (x <= x1) : (x += 1) {
            var inside = false;
            if (x >= x0 and x <= x1 and y >= y0 and y <= y1) {
                inside = true;
                if (x < x0 + r and y < y0 + r) {
                    const dx = @as(i32, x0 + r) - x;
                    const dy = @as(i32, y0 + r) - y;
                    if (dx * dx + dy * dy > r * r) inside = false;
                } else if (x > x1 - r and y < y0 + r) {
                    const dx = x - @as(i32, x1 - r);
                    const dy = @as(i32, y0 + r) - y;
                    if (dx * dx + dy * dy > r * r) inside = false;
                } else if (x < x0 + r and y > y1 - r) {
                    const dx = @as(i32, x0 + r) - x;
                    const dy = y - @as(i32, y1 - r);
                    if (dx * dx + dy * dy > r * r) inside = false;
                } else if (x > x1 - r and y > y1 - r) {
                    const dx = x - @as(i32, x1 - r);
                    const dy = y - @as(i32, y1 - r);
                    if (dx * dx + dy * dy > r * r) inside = false;
                }
            }
            if (inside) {
                const on_border = (x == x0 or x == x1 or y == y0 or y == y1);
                var on_corner = false;
                if (x < x0 + r and y < y0 + r) {
                    const dx = @as(i32, x0 + r) - x;
                    const dy = @as(i32, y0 + r) - y;
                    const dist2 = dx * dx + dy * dy;
                    on_corner = (dist2 >= (r - 1) * (r - 1) and dist2 <= r * r);
                } else if (x > x1 - r and y < y0 + r) {
                    const dx = x - @as(i32, x1 - r);
                    const dy = @as(i32, y0 + r) - y;
                    const dist2 = dx * dx + dy * dy;
                    on_corner = (dist2 >= (r - 1) * (r - 1) and dist2 <= r * r);
                } else if (x < x0 + r and y > y1 - r) {
                    const dx = @as(i32, x0 + r) - x;
                    const dy = y - @as(i32, y1 - r);
                    const dist2 = dx * dx + dy * dy;
                    on_corner = (dist2 >= (r - 1) * (r - 1) and dist2 <= r * r);
                } else if (x > x1 - r and y > y1 - r) {
                    const dx = x - @as(i32, x1 - r);
                    const dy = y - @as(i32, y1 - r);
                    const dist2 = dx * dx + dy * dy;
                    on_corner = (dist2 >= (r - 1) * (r - 1) and dist2 <= r * r);
                }
                if (on_border or on_corner) {
                    putPixel(buf, buf_w, x, y, stroke);
                } else {
                    putPixel(buf, buf_w, x, y, fill);
                }
            }
        }
    }
}

// ── Component renderers ───────────────────────────────────────────────

fn renderBadge(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    const pill_w: i32 = 90;
    const pill_h: i32 = 24;
    fillRoundedRect(buf, buf_w, x, y, x + pill_w, y + pill_h, 12, theme.accent);
    drawNumber(buf, buf_w, x + 10, y + 9, 128, 1, .{ 255, 255, 255, 255 });
    drawString(buf, buf_w, x + 42, y + 9, "INBOX", 1, .{ 255, 255, 255, 255 });
}

fn renderButton(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, variant: enum { default, primary, destructive }) void {
    const bw: i32 = 100;
    const bh: i32 = 28;
    const label = switch (variant) {
        .default => "CLICK",
        .primary => "SAVE",
        .destructive => "DELETE",
    };
    const fill: RGBA = switch (variant) {
        .default => theme.bg,
        .primary => theme.accent,
        .destructive => theme.red,
    };
    const stroke: RGBA = switch (variant) {
        .default => theme.border,
        .primary => theme.accent,
        .destructive => theme.red,
    };
    const text_col: RGBA = switch (variant) {
        .default => theme.fg,
        .primary => .{ 255, 255, 255, 255 },
        .destructive => .{ 255, 255, 255, 255 },
    };
    drawBorderedRoundedRect(buf, buf_w, x, y, x + bw, y + bh, 6, fill, stroke);
    drawString(buf, buf_w, x + 15, y + 10, label, 1, text_col);
}

fn renderCheckbox(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, state: enum { checked, unchecked, indeterminate }) void {
    const sz: i32 = 18;
    const border_col = switch (state) {
        .unchecked => theme.border,
        .checked => theme.accent,
        .indeterminate => theme.accent,
    };
    fillRoundedRect(buf, buf_w, x, y, x + sz, y + sz, 4, theme.bg);
    fillHLine(buf, buf_w, x + 1, x + sz - 1, y, border_col);
    fillHLine(buf, buf_w, x + 1, x + sz - 1, y + sz, border_col);
    fillVLine(buf, buf_w, x, y + 1, y + sz - 1, border_col);
    fillVLine(buf, buf_w, x + sz, y + 1, y + sz - 1, border_col);

    switch (state) {
        .checked => {
            fillRoundedRect(buf, buf_w, x + 1, y + 1, x + sz - 1, y + sz - 1, 3, theme.accent);
            putPixel(buf, buf_w, x + 5, y + 9, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 6, y + 10, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 7, y + 11, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 8, y + 10, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 9, y + 9, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 10, y + 8, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 11, y + 7, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 12, y + 6, .{ 255, 255, 255, 255 });
            putPixel(buf, buf_w, x + 13, y + 5, .{ 255, 255, 255, 255 });
        },
        .unchecked => {},
        .indeterminate => {
            fillHLine(buf, buf_w, x + 4, x + sz - 4, y + sz / 2, .{ 255, 255, 255, 255 });
        },
    }
}

fn renderSwitch(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, on: bool) void {
    const track_w: i32 = 42;
    const track_h: i32 = 24;
    const thumb_r: i32 = 10;
    const track_color = if (on) theme.accent else theme.border;
    fillRoundedRect(buf, buf_w, x, y, x + track_w, y + track_h, 12, track_color);
    const thumb_x = if (on) x + track_w - thumb_r - 4 else x + thumb_r + 4;
    fillCircle(buf, buf_w, thumb_x, y + track_h / 2, thumb_r, .{ 255, 255, 255, 255 });
}

fn renderProgress(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, determinate: bool, now_ms: i64) void {
    const pw: i32 = 200;
    const ph: i32 = 8;
    fillRoundedRect(buf, buf_w, x, y, x + pw, y + ph, 4, theme.bg_secondary);
    if (determinate) {
        const fill_w: i32 = pw * 60 / 100;
        fillRoundedRect(buf, buf_w, x, y, x + fill_w, y + ph, 4, theme.accent);
    } else {
        const offset: i32 = @intCast(@mod(@divTrunc(now_ms, 50), @as(i64, pw)));
        var stripe_x: i32 = -pw;
        while (stripe_x < pw * 2) : (stripe_x += 20) {
            const sx = x + stripe_x + offset;
            const sw: i32 = 10;
            if (sx + sw > x and sx < x + pw) {
                const clipped_x = if (sx < x) x else sx;
                const clipped_end = if (sx + sw > x + pw) x + pw else sx + sw;
                fillRoundedRect(buf, buf_w, clipped_x, y, clipped_end, y + ph, 2, theme.accent);
            }
        }
    }
}

fn renderSlider(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, value: f32) void {
    const sw: i32 = 200;
    const sh: i32 = 6;
    const thumb_r: i32 = 8;
    fillRoundedRect(buf, buf_w, x, y + thumb_r - sh / 2, x + sw, y + thumb_r + sh / 2, 3, theme.bg_secondary);
    const filled: i32 = @intFromFloat(@as(f32, @floatFromInt(sw)) * value);
    fillRoundedRect(buf, buf_w, x, y + thumb_r - sh / 2, x + filled, y + thumb_r + sh / 2, 3, theme.accent);
    const thumb_x = x + filled;
    fillCircle(buf, buf_w, thumb_x, y + thumb_r, thumb_r, theme.accent);
    fillCircle(buf, buf_w, thumb_x, y + thumb_r, thumb_r - 2, .{ 255, 255, 255, 255 });
}

fn renderSpinner(buf: []u8, buf_w: usize, theme: Theme, cx: i32, cy: i32, now_ms: i64) void {
    const radius: i32 = 12;
    const blade_count: i32 = 8;
    const phase = @divTrunc(now_ms, 100);
    var i: i32 = 0;
    while (i < blade_count) : (i += 1) {
        const angle = (@as(f32, @floatFromInt(i)) + @as(f32, @floatFromInt(@mod(phase, @as(i64, blade_count))))) / @as(f32, blade_count) * 3.14159 * 2.0;
        const alpha_f = 1.0 - @as(f32, @floatFromInt(i)) / @as(f32, blade_count);
        const alpha: u8 = @intFromFloat(alpha_f * 255.0);
        const inner_r: f32 = @floatFromInt(radius - 3);
        const outer_r: f32 = @floatFromInt(radius);
        const x1 = cx + @as(i32, @intFromFloat(@cos(angle) * inner_r));
        const y1 = cy + @as(i32, @intFromFloat(@sin(angle) * inner_r));
        const x2 = cx + @as(i32, @intFromFloat(@cos(angle) * outer_r));
        const y2 = cy + @as(i32, @intFromFloat(@sin(angle) * outer_r));
        const steps = 4;
        var s: i32 = 0;
        while (s <= steps) : (s += 1) {
            const t: f32 = @as(f32, @floatFromInt(s)) / @as(f32, steps);
            const px = lerp(x1, x2, t);
            const py = lerp(y1, y2, t);
            putPixel(buf, buf_w, px, py, colorAlpha(theme.fg, alpha));
            putPixel(buf, buf_w, px + 1, py, colorAlpha(theme.fg, alpha));
            putPixel(buf, buf_w, px, py + 1, colorAlpha(theme.fg, alpha));
        }
    }
}

fn renderStepper(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, value: i32) void {
    const bw: i32 = 28;
    const bh: i32 = 24;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + bw, y + bh, 6, theme.bg, theme.border);
    fillHLine(buf, buf_w, x + 9, x + 19, y + 12, theme.fg);
    drawBorderedRoundedRect(buf, buf_w, x + bw + 2, y, x + bw * 2 + 2, y + bh, 6, theme.bg, theme.border);
    fillHLine(buf, buf_w, x + bw + 11, x + bw + 21, y + 12, theme.fg);
    fillVLine(buf, buf_w, x + bw + 16, y + 7, y + 17, theme.fg);
    const val_str: []const u8 = switch (value) {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        else => "?",
    };
    drawString(buf, buf_w, x + bw + 8, y + 9, val_str, 1, theme.fg);
}

fn renderSeparator(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    fillHLine(buf, buf_w, x, x + LOGICAL_W - 20, y, theme.border);
}

fn renderLabel(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, text: []const u8, scale: i32) void {
    drawString(buf, buf_w, x, y, text, scale, theme.fg);
}

fn renderGlassPanel(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    const pw: i32 = 260;
    const ph: i32 = 50;
    const bg_alpha: u8 = if (theme.bg[0] == 255) @as(u8, 180) else @as(u8, 160);
    fillRoundedRect(buf, buf_w, x, y, x + pw, y + ph, 10, colorAlpha(theme.bg, bg_alpha));
    const border_alpha: u8 = if (theme.bg[0] == 255) @as(u8, 100) else @as(u8, 80);
    fillHLine(buf, buf_w, x + 10, x + pw - 10, y, colorAlpha(theme.border, border_alpha));
    fillHLine(buf, buf_w, x + 10, x + pw - 10, y + ph, colorAlpha(theme.border, border_alpha));
    fillVLine(buf, buf_w, x, y + 10, y + ph - 10, colorAlpha(theme.border, border_alpha));
    fillVLine(buf, buf_w, x + pw, y + 10, y + ph - 10, colorAlpha(theme.border, border_alpha));
    drawString(buf, buf_w, x + 10, y + 10, "GLASS PANEL", 1, theme.fg);
}

fn renderHelpButton(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    const r: i32 = 12;
    fillCircle(buf, buf_w, x + r, y + r, r, theme.accent);
    // "?" glyph
    putPixel(buf, buf_w, x + r - 2, y + r - 4, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r - 1, y + r - 5, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r, y + r - 5, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r + 1, y + r - 5, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r + 2, y + r - 4, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r + 2, y + r - 3, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r + 1, y + r - 2, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r, y + r - 1, .{ 255, 255, 255, 255 });
    putPixel(buf, buf_w, x + r, y + r + 2, .{ 255, 255, 255, 255 });
}

fn renderRadioGroup(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, selected: u8) void {
    const labels = [_][]const u8{ "ALPHA", "BETA", "GAMMA" };
    var i: u8 = 0;
    while (i < 3) : (i += 1) {
        const ry = y + @as(i32, @intCast(i)) * 24;
        const radio_r: i32 = 8;
        const cx = x + radio_r;
        fillCircle(buf, buf_w, cx, ry + radio_r, radio_r, theme.border);
        fillCircle(buf, buf_w, cx, ry + radio_r, radio_r - 2, theme.bg);
        if (i == selected) {
            fillCircle(buf, buf_w, cx, ry + radio_r, radio_r - 3, theme.accent);
        }
        drawString(buf, buf_w, x + 24, ry + radio_r - 3, labels[i], 1, theme.fg);
    }
}

fn renderSearchField(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, search_len: usize) void {
    const fw: i32 = 240;
    const fh: i32 = 28;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + fw, y + fh, 14, theme.bg, theme.border);
    const icon_cx = x + 16;
    const icon_cy = y + 14;
    fillCircle(buf, buf_w, icon_cx, icon_cy, 5, theme.border);
    fillCircle(buf, buf_w, icon_cx, icon_cy, 3, theme.bg);
    putPixel(buf, buf_w, icon_cx + 4, icon_cy + 4, theme.border);
    putPixel(buf, buf_w, icon_cx + 5, icon_cy + 5, theme.border);
    putPixel(buf, buf_w, icon_cx + 6, icon_cy + 6, theme.border);
    if (search_len > 0) {
        fillVLine(buf, buf_w, x + 30, y + 8, y + 20, theme.accent);
    } else {
        drawString(buf, buf_w, x + 30, y + 9, "SEARCH", 1, theme.border);
    }
}

fn renderSecureField(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, content_len: usize) void {
    const fw: i32 = 240;
    const fh: i32 = 28;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + fw, y + fh, 6, theme.bg, theme.border);
    var i: usize = 0;
    while (i < content_len and i < 10) : (i += 1) {
        fillCircle(buf, buf_w, x + 12 + @as(i32, @intCast(i)) * 12, y + 14, 3, theme.fg);
    }
    if (content_len == 0) {
        drawString(buf, buf_w, x + 10, y + 9, "PASSWORD", 1, theme.border);
    }
}

fn renderTextField(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, content_len: usize) void {
    const fw: i32 = 240;
    const fh: i32 = 28;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + fw, y + fh, 6, theme.bg, theme.border);
    if (content_len > 0) {
        fillVLine(buf, buf_w, x + 10 + @as(i32, @intCast(content_len)) * 6, y + 8, y + 20, theme.accent);
    } else {
        drawString(buf, buf_w, x + 10, y + 9, "PLACEHOLDER", 1, theme.border);
    }
}

fn renderSelect(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, index: u8, is_open: bool) void {
    const fw: i32 = 240;
    const fh: i32 = 28;
    const labels = [_][]const u8{ "OPTION A", "OPTION B", "OPTION C" };
    drawBorderedRoundedRect(buf, buf_w, x, y, x + fw, y + fh, 6, theme.bg, theme.border);
    drawString(buf, buf_w, x + 10, y + 9, labels[index], 1, theme.fg);
    // Dropdown arrow
    putPixel(buf, buf_w, x + fw - 16, y + 12, theme.fg);
    putPixel(buf, buf_w, x + fw - 15, y + 13, theme.fg);
    putPixel(buf, buf_w, x + fw - 14, y + 14, theme.fg);
    putPixel(buf, buf_w, x + fw - 13, y + 13, theme.fg);
    putPixel(buf, buf_w, x + fw - 12, y + 12, theme.fg);
    if (is_open) {
        var i: u8 = 0;
        while (i < 3) : (i += 1) {
            const my = y + fh + @as(i32, @intCast(i)) * (fh - 4);
            fillRoundedRect(buf, buf_w, x, my, x + fw, my + fh - 4, 0, theme.bg);
            const text_col = if (i == index) theme.accent else theme.fg;
            drawString(buf, buf_w, x + 10, my + 9, labels[i], 1, text_col);
        }
    }
}

fn renderPopupButton(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, index: u8, is_open: bool) void {
    const bw: i32 = 160;
    const bh: i32 = 28;
    const labels = [_][]const u8{ "FILE", "EDIT", "VIEW" };
    drawBorderedRoundedRect(buf, buf_w, x, y, x + bw, y + bh, 6, theme.bg, theme.border);
    drawString(buf, buf_w, x + 10, y + 9, labels[index], 1, theme.fg);
    putPixel(buf, buf_w, x + bw - 16, y + 12, theme.fg);
    putPixel(buf, buf_w, x + bw - 15, y + 13, theme.fg);
    putPixel(buf, buf_w, x + bw - 14, y + 14, theme.fg);
    putPixel(buf, buf_w, x + bw - 13, y + 13, theme.fg);
    putPixel(buf, buf_w, x + bw - 12, y + 12, theme.fg);
    if (is_open) {
        var i: u8 = 0;
        while (i < 3) : (i += 1) {
            const my = y + bh + @as(i32, @intCast(i)) * (bh - 4);
            fillRoundedRect(buf, buf_w, x, my, x + bw, my + bh - 4, 0, theme.bg);
            drawString(buf, buf_w, x + 10, my + 9, labels[i], 1, theme.fg);
        }
    }
}

fn renderSegmentedControl(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32, selected: u8) void {
    const seg_w: i32 = 80;
    const seg_h: i32 = 28;
    const total_w = seg_w * 3;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + total_w, y + seg_h, 6, theme.bg, theme.border);
    const labels = [_][]const u8{ "FIRST", "SECOND", "THIRD" };
    var i: u8 = 0;
    while (i < 3) : (i += 1) {
        const sx = x + @as(i32, @intCast(i)) * seg_w;
        if (i == selected) {
            fillRoundedRect(buf, buf_w, sx + 1, y + 1, sx + seg_w - 1, y + seg_h - 1, 5, theme.accent);
            drawString(buf, buf_w, sx + 10, y + 9, labels[i], 1, .{ 255, 255, 255, 255 });
        } else {
            drawString(buf, buf_w, sx + 10, y + 9, labels[i], 1, theme.fg);
        }
        if (i > 0) {
            fillVLine(buf, buf_w, sx, y + 4, y + seg_h - 4, theme.border);
        }
    }
}

fn renderBox(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    const bw: i32 = 260;
    const bh: i32 = 40;
    drawBorderedRoundedRect(buf, buf_w, x, y, x + bw, y + bh, 6, theme.bg, theme.border);
    drawString(buf, buf_w, x + 10, y + 10, "GROUPED BOX", 1, theme.fg);
}

fn renderPanel(buf: []u8, buf_w: usize, theme: Theme, x: i32, y: i32) void {
    const pw: i32 = 260;
    const ph: i32 = 40;
    fillRoundedRect(buf, buf_w, x, y, x + pw, y + ph, 6, theme.bg_secondary);
    drawString(buf, buf_w, x + 10, y + 10, "SURFACE PANEL", 1, theme.fg);
}

// ── Overview renderer ─────────────────────────────────────────────────

fn renderOverview(buf: []u8, buf_w: usize, inst: *Instance, now_ms: i64) void {
    const theme = if (inst.dark) DARK else LIGHT;
    const x0: i32 = 10;
    var y0: i32 = 10;
    const sy: i32 = @intFromFloat(inst.scroll_y);

    renderBadge(buf, buf_w, theme, x0, y0 - sy);
    y0 = 50;
    drawString(buf, buf_w, x0, y0 - sy, "BADGE", 1, theme.border);
    y0 = 70;

    renderButton(buf, buf_w, theme, x0, y0 - sy, .default);
    renderButton(buf, buf_w, theme, x0 + 110, y0 - sy, .primary);
    renderButton(buf, buf_w, theme, x0 + 220, y0 - sy, .destructive);
    y0 = 110;
    drawString(buf, buf_w, x0, y0 - sy, "BUTTONS", 1, theme.border);
    y0 = 130;

    renderCheckbox(buf, buf_w, theme, x0, y0 - sy, .unchecked);
    drawString(buf, buf_w, x0 + 26, y0 - sy + 1, "UNCHECKED", 1, theme.fg);
    renderCheckbox(buf, buf_w, theme, x0 + 100, y0 - sy, .checked);
    drawString(buf, buf_w, x0 + 126, y0 - sy + 1, "CHECKED", 1, theme.fg);
    renderCheckbox(buf, buf_w, theme, x0 + 200, y0 - sy, .indeterminate);
    drawString(buf, buf_w, x0 + 226, y0 - sy + 1, "INDET", 1, theme.fg);
    y0 = 160;
    drawString(buf, buf_w, x0, y0 - sy, "CHECKBOXES", 1, theme.border);
    y0 = 180;

    renderSwitch(buf, buf_w, theme, x0, y0 - sy, false);
    drawString(buf, buf_w, x0 + 52, y0 - sy + 3, "OFF", 1, theme.fg);
    renderSwitch(buf, buf_w, theme, x0 + 100, y0 - sy, true);
    drawString(buf, buf_w, x0 + 152, y0 - sy + 3, "ON", 1, theme.fg);
    y0 = 220;
    drawString(buf, buf_w, x0, y0 - sy, "SWITCHES", 1, theme.border);
    y0 = 240;

    renderProgress(buf, buf_w, theme, x0, y0 - sy, true, now_ms);
    renderProgress(buf, buf_w, theme, x0, y0 - sy + 20, false, now_ms);
    y0 = 280;
    drawString(buf, buf_w, x0, y0 - sy, "PROGRESS", 1, theme.border);
    y0 = 300;

    renderSlider(buf, buf_w, theme, x0, y0 - sy, inst.slider_value);
    y0 = 330;
    drawString(buf, buf_w, x0, y0 - sy, "SLIDER", 1, theme.border);
    y0 = 350;

    renderSpinner(buf, buf_w, theme, x0 + 12, y0 - sy + 12, now_ms);
    drawString(buf, buf_w, x0 + 30, y0 - sy + 6, "SPINNER", 1, theme.fg);
    y0 = 380;
    drawString(buf, buf_w, x0, y0 - sy, "SPINNER", 1, theme.border);
    y0 = 400;

    renderStepper(buf, buf_w, theme, x0, y0 - sy, inst.stepper_value);
    y0 = 440;
    drawString(buf, buf_w, x0, y0 - sy, "STEPPER", 1, theme.border);
    y0 = 460;

    renderSeparator(buf, buf_w, theme, x0, y0 - sy);
    y0 = 480;

    renderLabel(buf, buf_w, theme, x0, y0 - sy, "LARGE TITLE", 2);
    y0 = 510;
    renderLabel(buf, buf_w, theme, x0, y0 - sy, "TITLE 1", 2);
    y0 = 540;
    renderLabel(buf, buf_w, theme, x0, y0 - sy, "BODY TEXT", 1);
    y0 = 555;
    renderLabel(buf, buf_w, theme, x0, y0 - sy, "CAPTION", 1);
    y0 = 575;
    drawString(buf, buf_w, x0, y0 - sy, "LABELS", 1, theme.border);
    y0 = 595;

    renderGlassPanel(buf, buf_w, theme, x0, y0 - sy);
    y0 = 660;
    drawString(buf, buf_w, x0, y0 - sy, "GLASS PANEL", 1, theme.border);
    y0 = 680;

    renderHelpButton(buf, buf_w, theme, x0 + 12, y0 - sy + 12);
    drawString(buf, buf_w, x0 + 40, y0 - sy + 6, "HELP BUTTON", 1, theme.fg);
    y0 = 710;
    drawString(buf, buf_w, x0, y0 - sy, "HELP", 1, theme.border);
    y0 = 730;

    renderRadioGroup(buf, buf_w, theme, x0, y0 - sy, inst.radio_selected);
    y0 = 810;
    drawString(buf, buf_w, x0, y0 - sy, "RADIO GROUP", 1, theme.border);
    y0 = 830;

    renderSearchField(buf, buf_w, theme, x0, y0 - sy, inst.search_len);
    y0 = 870;
    drawString(buf, buf_w, x0, y0 - sy, "SEARCH FIELD", 1, theme.border);
    y0 = 890;

    renderSecureField(buf, buf_w, theme, x0, y0 - sy, inst.secure_field_len);
    y0 = 930;
    drawString(buf, buf_w, x0, y0 - sy, "SECURE FIELD", 1, theme.border);
    y0 = 950;

    renderTextField(buf, buf_w, theme, x0, y0 - sy, inst.text_field_len);
    y0 = 990;
    drawString(buf, buf_w, x0, y0 - sy, "TEXT FIELD", 1, theme.border);
    y0 = 1010;

    renderSelect(buf, buf_w, theme, x0, y0 - sy, inst.select_index, inst.select_open);
    y0 = 1050;
    drawString(buf, buf_w, x0, y0 - sy, "SELECT", 1, theme.border);
    y0 = 1070;

    renderPopupButton(buf, buf_w, theme, x0, y0 - sy, inst.popup_index, inst.popup_open);
    y0 = 1110;
    drawString(buf, buf_w, x0, y0 - sy, "POPUP BUTTON", 1, theme.border);
    y0 = 1130;

    renderSegmentedControl(buf, buf_w, theme, x0, y0 - sy, inst.segmented_index);
    y0 = 1170;
    drawString(buf, buf_w, x0, y0 - sy, "SEGMENTED", 1, theme.border);
    y0 = 1190;

    renderBox(buf, buf_w, theme, x0, y0 - sy);
    y0 = 1240;
    drawString(buf, buf_w, x0, y0 - sy, "BOX", 1, theme.border);
    y0 = 1260;

    renderPanel(buf, buf_w, theme, x0, y0 - sy);
    y0 = 1310;
    drawString(buf, buf_w, x0, y0 - sy, "PANEL", 1, theme.border);
}

// ── WASM exported functions ───────────────────────────────────────────

export fn preview_alloc(len: u32) u32 {
    const result = bumpAlloc(@intCast(len));
    if (result) |ptr| {
        render_buf_ptr = ptr;
        return @intCast(ptr);
    }
    return 0;
}

export fn preview_free(ptr: u32) void {
    _ = ptr;
}

export fn preview_create(name_ptr: u32, name_len: u32, dark: u32) u32 {
    if (instance_count >= instances.len) return 0;

    const id: u32 = @intCast(instance_count);
    const inst = &instances[instance_count];

    const name_slice = @as([*]u8, @ptrFromInt(mem_base + name_ptr))[0..name_len];
    const allocated_name = bumpAlloc(name_len) orelse return 0;
    const dst = @as([*]u8, @ptrFromInt(allocated_name))[0..name_len];
    for (name_slice, 0..) |ch, i| {
        dst[i] = ch;
    }
    inst.name = dst;

    inst.dark = dark != 0;
    inst.now_ms = 0;
    inst.cursor = CURSOR_DEFAULT;
    inst.scroll_x = 0;
    inst.scroll_y = 0;
    inst.ptr_x = 0;
    inst.ptr_y = 0;
    inst.ptr_down = false;
    inst.checkbox_checked = true;
    inst.checkbox_indeterminate = false;
    inst.switch_on = true;
    inst.progress_value = 0.6;
    inst.slider_value = 0.4;
    inst.radio_selected = 0;
    inst.text_field_len = 0;
    inst.secure_field_len = 0;
    inst.search_len = 0;
    inst.select_open = false;
    inst.select_index = 0;
    inst.popup_open = false;
    inst.popup_index = 0;
    inst.segmented_index = 0;
    inst.stepper_value = 0;

    instance_count += 1;
    return id + 1;
}

export fn preview_render(preview_id: u32, scale: u32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    const inst = &instances[preview_id - 1];

    _ = scale;

    if (render_buf_ptr == 0) return;

    const w: usize = @intCast(LOGICAL_W);
    const h: usize = @intCast(LOGICAL_H);
    const buf_ptr = render_buf_ptr;
    const buf: []u8 = @as([*]u8, @ptrFromInt(mem_base + buf_ptr))[0 .. w * h * 4];

    const theme = if (inst.dark) DARK else LIGHT;
    fillRoundedRect(buf, w, 0, 0, @intCast(w - 1), @intCast(h - 1), 0, theme.bg);

    renderOverview(buf, w, inst, inst.now_ms);
}

export fn preview_pointer(preview_id: u32, kind: u32, x: f32, y: f32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    const inst = &instances[preview_id - 1];

    inst.ptr_x = x;
    inst.ptr_y = y;

    if (kind == PTR_DOWN) {
        inst.ptr_down = true;
        const ix: i32 = @intFromFloat(x);
        const iy: i32 = @as(i32, @intFromFloat(y)) + @as(i32, @intFromFloat(inst.scroll_y));

        if (iy >= 130 and iy <= 148) {
            if (ix >= 10 and ix <= 28) {
                inst.checkbox_checked = false;
                inst.checkbox_indeterminate = false;
                inst.cursor = CURSOR_POINTER;
            } else if (ix >= 110 and ix <= 128) {
                inst.checkbox_checked = true;
                inst.checkbox_indeterminate = false;
                inst.cursor = CURSOR_POINTER;
            } else if (ix >= 210 and ix <= 228) {
                inst.checkbox_checked = false;
                inst.checkbox_indeterminate = true;
                inst.cursor = CURSOR_POINTER;
            }
        }

        if (iy >= 180 and iy <= 204 and ix >= 10 and ix <= 52) {
            inst.switch_on = !inst.switch_on;
            inst.cursor = CURSOR_POINTER;
        }

        if (iy >= 730 and iy <= 800) {
            const rel_y = iy - 730;
            if (rel_y < 24) {
                inst.radio_selected = 0;
            } else if (rel_y < 48) {
                inst.radio_selected = 1;
            } else if (rel_y < 72) {
                inst.radio_selected = 2;
            }
            inst.cursor = CURSOR_POINTER;
        }

        if (iy >= 1130 and iy <= 1158) {
            const rel_x = ix - 10;
            if (rel_x < 80) {
                inst.segmented_index = 0;
            } else if (rel_x < 160) {
                inst.segmented_index = 1;
            } else if (rel_x < 240) {
                inst.segmented_index = 2;
            }
            inst.cursor = CURSOR_POINTER;
        }

        if (iy >= 1010 and iy <= 1038 and ix >= 10 and ix <= 250) {
            inst.select_open = !inst.select_open;
            inst.cursor = CURSOR_POINTER;
        } else if (inst.select_open and iy >= 1038 and iy <= 1090) {
            const rel_y = iy - 1038;
            if (rel_y < 24) {
                inst.select_index = 0;
            } else if (rel_y < 48) {
                inst.select_index = 1;
            } else if (rel_y < 72) {
                inst.select_index = 2;
            }
            inst.select_open = false;
            inst.cursor = CURSOR_POINTER;
        }

        if (iy >= 1070 and iy <= 1098 and ix >= 10 and ix <= 170) {
            inst.popup_open = !inst.popup_open;
            inst.cursor = CURSOR_POINTER;
        } else if (inst.popup_open and iy >= 1098 and iy <= 1150) {
            const rel_y = iy - 1098;
            if (rel_y < 24) {
                inst.popup_index = 0;
            } else if (rel_y < 48) {
                inst.popup_index = 1;
            } else if (rel_y < 72) {
                inst.popup_index = 2;
            }
            inst.popup_open = false;
            inst.cursor = CURSOR_POINTER;
        }

        if (iy >= 400 and iy <= 424) {
            if (ix >= 10 and ix <= 38) {
                inst.stepper_value -|= 1;
                inst.cursor = CURSOR_POINTER;
            } else if (ix >= 40 and ix <= 68) {
                inst.stepper_value +|= 1;
                inst.cursor = CURSOR_POINTER;
            }
        }
    } else if (kind == PTR_UP) {
        inst.ptr_down = false;
    } else if (kind == PTR_MOVE) {
        inst.cursor = CURSOR_DEFAULT;
        const iy: i32 = @as(i32, @intFromFloat(y)) + @as(i32, @intFromFloat(inst.scroll_y));
        if (iy >= 180 and iy <= 204 and @as(i32, @intFromFloat(x)) >= 10 and @as(i32, @intFromFloat(x)) <= 52) {
            inst.cursor = CURSOR_POINTER;
        }
    }
}

export fn preview_scroll(preview_id: u32, dx: f32, dy: f32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    const inst = &instances[preview_id - 1];
    inst.scroll_x += dx;
    inst.scroll_y += dy;
    if (inst.scroll_y < 0) inst.scroll_y = 0;
    if (inst.scroll_y > 1200) inst.scroll_y = 1200;
}

export fn preview_key(preview_id: u32, key_ptr: u32, key_len: u32, modifiers: u32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    _ = modifiers;
    const inst = &instances[preview_id - 1];
    const key_bytes = @as([*]u8, @ptrFromInt(mem_base + key_ptr))[0..key_len];
    if (key_len == 1) {
        const ch = key_bytes[0];
        if (ch == '\t') {
            inst.segmented_index = (inst.segmented_index + 1) % 3;
        } else if (ch == ' ') {
            inst.switch_on = !inst.switch_on;
        }
    }
}

export fn preview_text(preview_id: u32, text_ptr: u32, text_len: u32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    const inst = &instances[preview_id - 1];
    const text_bytes = @as([*]u8, @ptrFromInt(mem_base + text_ptr))[0..text_len];
    for (text_bytes) |ch| {
        if (inst.text_field_len < inst.text_field_buf.len) {
            inst.text_field_buf[inst.text_field_len] = ch;
            inst.text_field_len += 1;
        }
    }
}

export fn preview_destroy(preview_id: u32) void {
    _ = preview_id;
}

export fn preview_set_now_ms(preview_id: u32, ms: u64) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    instances[preview_id - 1].now_ms = @intCast(ms);
}

export fn preview_set_theme(preview_id: u32, dark: u32) void {
    if (preview_id == 0 or preview_id > instance_count) return;
    instances[preview_id - 1].dark = dark != 0;
}

export fn preview_logical_width(preview_id: u32) u32 {
    if (preview_id == 0 or preview_id > instance_count) return 0;
    return LOGICAL_W;
}

export fn preview_logical_height(preview_id: u32) u32 {
    if (preview_id == 0 or preview_id > instance_count) return 0;
    return LOGICAL_H;
}

export fn preview_cursor(preview_id: u32) u32 {
    if (preview_id == 0 or preview_id > instance_count) return CURSOR_DEFAULT;
    return instances[preview_id - 1].cursor;
}

export fn preview_pixel_byte_len(preview_id: u32) u32 {
    if (preview_id == 0 or preview_id > instance_count) return 0;
    return @intCast(LOGICAL_W * LOGICAL_H * 4);
}

export fn preview_status(preview_id: u32) u32 {
    if (preview_id == 0 or preview_id > instance_count) return 0;
    return 1;
}

// ── WASM memory initialization ────────────────────────────────────────

export fn _initialize() void {
    mem_base = 0;
    mem_len = INITIAL_PAGES * PAGE_SIZE;
    bump_ptr = 1024 * 1024;
}

export fn _start() void {}
