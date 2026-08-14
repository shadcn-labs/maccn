// maccn native-sdk: Secure Field component
// A masked password input field.

// Markup:
//   <secure-field placeholder="Password" on-submit="login" />
//   <secure-field disabled="true" />

const std = @import("std");
const ui = @import("ui");

pub fn secureField(opts: struct {
    placeholder: ?[]const u8 = null,
    disabled: bool = false,
    on_submit: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.secure_field, .{
        .placeholder = opts.placeholder,
        .disabled = opts.disabled,
        .on_submit = opts.on_submit,
    }, .{});
}

test "secure field renders" {
    const el = secureField(.{ .placeholder = "Password" });
    try std.testing.expectEqual(@as(ui.ElementType, .secure_field), el.type);
}
