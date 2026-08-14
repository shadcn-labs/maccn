// maccn native-sdk: Spinner component
// An animated loading indicator with spinning blades.

// Markup:
//   <spinner />
//   <spinner size="sm" />

const std = @import("std");
const ui = @import("ui");

pub fn spinner(opts: struct {
    size: enum { sm, default } = .default,
    color: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.spinner, .{
        .size = opts.size,
        .color = opts.color,
    }, .{});
}

test "spinner renders" {
    const el = spinner(.{});
    try std.testing.expectEqual(@as(ui.ElementType, .spinner), el.type);
}
