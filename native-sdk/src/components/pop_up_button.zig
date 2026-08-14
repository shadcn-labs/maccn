// maccn native-sdk: Pop-Up Button component
// A dropdown button that opens a popover menu.

// Markup:
//   <pop-up-button value="{format}" on-change="set_format">
//     <pop-up-item value="pdf">PDF</pop-up-item>
//     <pop-up-item value="png">PNG</pop-up-item>
//     <pop-up-item value="jpeg" disabled="true">JPEG</pop-up-item>
//   </pop-up-button>

const std = @import("std");
const ui = @import("ui");

pub fn popUpButton(opts: struct {
    value: ?[]const u8 = null,
    placeholder: ?[]const u8 = null,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
    children: anytype,
}) ui.Element {
    return ui.el(.pop_up_button, .{
        .value = opts.value,
        .placeholder = opts.placeholder,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
    }, opts.children);
}

pub fn popUpItem(opts: struct {
    value: []const u8,
    disabled: bool = false,
    separator: bool = false,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.pop_up_item, .{
        .value = opts.value,
        .disabled = opts.disabled,
        .separator = opts.separator,
        .text = opts.text,
    }, content);
}

test "pop-up button renders" {
    const el = popUpButton(.{ .value = "pdf" }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .pop_up_button), el.type);
}
