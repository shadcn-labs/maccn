// maccn native-sdk: Separator component
// A horizontal or vertical dividing line.

// Markup:
//   <separator />
//   <separator orientation="vertical" />

const std = @import("std");
const ui = @import("ui");

pub fn separator(opts: struct {
    orientation: enum { horizontal, vertical } = .horizontal,
}) ui.Element {
    return ui.el(.separator, .{
        .orientation = opts.orientation,
    }, .{});
}

test "separator renders" {
    const el = separator(.{});
    try std.testing.expectEqual(@as(ui.ElementType, .separator), el.type);
}
