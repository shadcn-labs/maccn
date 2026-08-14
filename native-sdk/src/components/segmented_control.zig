// maccn native-sdk: Segmented Control component
// A segmented control for switching between views.

// Markup:
//   <segmented-control value="{view}" on-change="set_view">
//     <segment value="list" selected="true">List</segment>
//     <segment value="grid">Grid</segment>
//     <segment value="columns">Columns</segment>
//   </segmented-control>

const std = @import("std");
const ui = @import("ui");

pub fn segmentedControl(opts: struct {
    value: ?[]const u8 = null,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
    children: anytype,
}) ui.Element {
    return ui.el(.segmented_control, .{
        .value = opts.value,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
    }, opts.children);
}

pub fn segment(opts: struct {
    value: []const u8,
    selected: bool = false,
    disabled: bool = false,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.segment, .{
        .value = opts.value,
        .selected = opts.selected,
        .disabled = opts.disabled,
        .text = opts.text,
    }, content);
}

test "segmented control renders" {
    const el = segmentedControl(.{}, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .segmented_control), el.type);
}
