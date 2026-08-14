// maccn native-sdk: Button component
// A text-bearing press control with macOS styling.

// Markup:
//   <button variant="primary" on-press="save">Save</button>
//   <button variant="destructive" on-press="delete">Delete</button>
//   <button size="sm" variant="outline">Small</button>

const std = @import("std");
const ui = @import("ui");

pub fn button(opts: struct {
    variant: enum { default, primary, secondary, outline, ghost, destructive } = .default,
    size: enum { sm, default, lg, icon } = .default,
    icon: ?[]const u8 = null,
    disabled: bool = false,
    on_press: ?[]const u8 = null,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.button(.{
        .variant = opts.variant,
        .size = opts.size,
        .icon = opts.icon,
        .disabled = opts.disabled,
        .on_press = opts.on_press,
    }, content);
}

test "button renders" {
    const el = button(.{ .text = "Save", .on_press = "save" }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .button), el.type);
}
