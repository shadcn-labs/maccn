// maccn native-sdk: Text Field component
// A text input with placeholder, prefix, and suffix support.

// Markup:
//   <text-field placeholder="you@example.com" on-submit="save_email" />
//   <text-field pill="true" placeholder="Search..." />
//   <text-field prefix="https://" suffix=".com" />

const std = @import("std");
const ui = @import("ui");

pub fn textField(opts: struct {
    placeholder: ?[]const u8 = null,
    pill: bool = false,
    prefix: ?[]const u8 = null,
    suffix: ?[]const u8 = null,
    radius: ?f32 = null,
    disabled: bool = false,
    on_submit: ?[]const u8 = null,
    on_change: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.text_field, .{
        .placeholder = opts.placeholder,
        .pill = opts.pill,
        .prefix = opts.prefix,
        .suffix = opts.suffix,
        .radius = opts.radius,
        .disabled = opts.disabled,
        .on_submit = opts.on_submit,
        .on_change = opts.on_change,
    }, .{});
}

test "text field renders" {
    const el = textField(.{ .placeholder = "Email" });
    try std.testing.expectEqual(@as(ui.ElementType, .text_field), el.type);
}
