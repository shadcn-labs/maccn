// maccn native-sdk: Checkbox component
// A tri-state checkbox with check and indeterminate marks.

// Markup:
//   <checkbox checked="true" on-change="toggle">Agree</checkbox>
//   <checkbox indeterminate="true">Partial</checkbox>

const std = @import("std");
const ui = @import("ui");

pub fn checkbox(opts: struct {
    checked: bool = false,
    indeterminate: bool = false,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.checkbox, .{
        .checked = opts.checked,
        .indeterminate = opts.indeterminate,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
        .text = opts.text,
    }, content);
}

test "checkbox renders" {
    const el = checkbox(.{ .text = "Agree", .checked = true }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .checkbox), el.type);
}
