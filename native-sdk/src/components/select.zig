// maccn native-sdk: Select component
// A dropdown that opens a popover menu.

// Markup:
//   <select value="{format}" on-change="set_format">
//     <select-item value="pdf">PDF</select-item>
//     <select-item value="png">PNG</select-item>
//     <select-item value="jpeg">JPEG</select-item>
//   </select>

const std = @import("std");
const ui = @import("ui");

pub fn select(opts: struct {
    value: ?[]const u8 = null,
    placeholder: ?[]const u8 = null,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
    children: anytype,
}) ui.Element {
    return ui.el(.select, .{
        .value = opts.value,
        .placeholder = opts.placeholder,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
    }, opts.children);
}

pub fn selectItem(opts: struct {
    value: []const u8,
    disabled: bool = false,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.select_item, .{
        .value = opts.value,
        .disabled = opts.disabled,
        .text = opts.text,
    }, content);
}

test "select renders" {
    const el = select(.{ .value = "pdf" }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .select), el.type);
}
