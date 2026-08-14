// maccn native-sdk: Search Field component
// A search input with magnifier icon and clear button.

// Markup:
//   <search-field placeholder="Search..." on-submit="search" />
//   <search-field disabled="true" />

const std = @import("std");
const ui = @import("ui");

pub fn searchField(opts: struct {
    placeholder: ?[]const u8 = null,
    disabled: bool = false,
    on_submit: ?[]const u8 = null,
    on_change: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.search_field, .{
        .placeholder = opts.placeholder,
        .disabled = opts.disabled,
        .on_submit = opts.on_submit,
        .on_change = opts.on_change,
    }, .{});
}

test "search field renders" {
    const el = searchField(.{ .placeholder = "Search..." });
    try std.testing.expectEqual(@as(ui.ElementType, .search_field), el.type);
}
