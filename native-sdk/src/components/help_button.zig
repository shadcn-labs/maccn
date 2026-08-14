// maccn native-sdk: Help Button component
// A circular help button with a "?" glyph.

// Markup:
//   <help-button on-press="show_help" />
//   <help-button size="sm" disabled="true" />

const std = @import("std");
const ui = @import("ui");

pub fn helpButton(opts: struct {
    size: enum { mini, small, default, large, extra_large } = .default,
    disabled: bool = false,
    on_press: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.help_button, .{
        .size = opts.size,
        .disabled = opts.disabled,
        .on_press = opts.on_press,
    }, .{});
}

test "help button renders" {
    const el = helpButton(.{ .on_press = "show_help" });
    try std.testing.expectEqual(@as(ui.ElementType, .help_button), el.type);
}
