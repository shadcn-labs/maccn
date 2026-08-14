// maccn native-sdk: Input component
// A text input with placeholder, prefix, and suffix support.

// Markup:
//   <input placeholder="you@example.com" on-submit="search" />
//   <input type="password" placeholder="Password" />
//   <input pill="true" placeholder="Search..." prefix="🔍" />

const std = @import("std");
const ui = @import("ui");

pub fn input(opts: struct {
    type: enum { text, password, email, url } = .text,
    placeholder: ?[]const u8 = null,
    pill: bool = false,
    prefix: ?[]const u8 = null,
    suffix: ?[]const u8 = null,
    disabled: bool = false,
    on_submit: ?[]const u8 = null,
    on_change: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.input, .{
        .type = opts.type,
        .placeholder = opts.placeholder,
        .pill = opts.pill,
        .prefix = opts.prefix,
        .suffix = opts.suffix,
        .disabled = opts.disabled,
        .on_submit = opts.on_submit,
        .on_change = opts.on_change,
    }, .{});
}

test "input renders" {
    const el = input(.{ .placeholder = "Email" });
    try std.testing.expectEqual(@as(ui.ElementType, .input), el.type);
}
