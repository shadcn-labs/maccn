// maccn native-sdk: Badge component
// A macOS sidebar count badge — translucent capsule with count or label.

// Markup:
//   <badge>128</badge>
//   <badge variant="secondary">New</badge>
//   <badge variant="outline">Outline</badge>
//   <badge variant="destructive">3</badge>

const std = @import("std");
const ui = @import("ui");

pub fn badge(opts: struct {
    variant: enum { default, secondary, outline, destructive } = .default,
    icon: ?[]const u8 = null,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.badge, .{
        .text = opts.text,
        .variant = opts.variant,
        .icon = opts.icon,
    }, content);
}

test "badge renders" {
    const el = badge(.{ .text = "128" }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .badge), el.type);
}
