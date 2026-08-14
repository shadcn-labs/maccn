// maccn native-sdk: Radio component
// A group of mutually exclusive radio buttons.

// Markup:
//   <radio-group axis="vertical" on-change="select_option">
//     <radio value="opt1" checked="true">Option 1</radio>
//     <radio value="opt2">Option 2</radio>
//     <radio value="opt3">Option 3</radio>
//   </radio-group>

const std = @import("std");
const ui = @import("ui");

pub fn radioGroup(opts: struct {
    axis: enum { horizontal, vertical } = .vertical,
    gap: ?f32 = null,
    on_change: ?[]const u8 = null,
    children: anytype,
}) ui.Element {
    return ui.column(.{
        .gap = opts.gap orelse 8,
    }, opts.children);
}

pub fn radio(opts: struct {
    value: []const u8,
    checked: bool = false,
    disabled: bool = false,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.radio, .{
        .value = opts.value,
        .checked = opts.checked,
        .disabled = opts.disabled,
        .text = opts.text,
    }, content);
}

test "radio renders" {
    const el = radio(.{ .value = "opt1", .text = "Option 1" }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .radio), el.type);
}
