// maccn native-sdk: Stepper component
// An increment/decrement control with up and down chevrons.

// Markup:
//   <stepper on-increment="inc" on-decrement="dec" />
//   <stepper disabled="true" />

const std = @import("std");
const ui = @import("ui");

pub fn stepper(opts: struct {
    disabled: bool = false,
    on_increment: ?[]const u8 = null,
    on_decrement: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.stepper, .{
        .disabled = opts.disabled,
        .on_increment = opts.on_increment,
        .on_decrement = opts.on_decrement,
    }, .{});
}

test "stepper renders" {
    const el = stepper(.{ .on_increment = "inc", .on_decrement = "dec" });
    try std.testing.expectEqual(@as(ui.ElementType, .stepper), el.type);
}
