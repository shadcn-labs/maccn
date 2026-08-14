// maccn native-sdk: Label component
// A text label with macOS typography styles.

// Markup:
//   <label style="title1">Hello World</label>
//   <label style="body" secondary="true">Muted text</label>

const std = @import("std");
const ui = @import("ui");

pub fn label(opts: struct {
    style: enum {
        large_title,
        title1,
        title2,
        title3,
        headline,
        body,
        callout,
        subheadline,
        footnote,
        caption1,
        caption2,
    } = .body,
    secondary: bool = false,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.text(.{
        .style = opts.style,
        .secondary = opts.secondary,
    }, opts.text);
}

test "label renders" {
    const el = label(.{ .text = "Hello", .style = .title1 }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .text), el.type);
}
