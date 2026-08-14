// maccn native-sdk: Switch component
// A toggle switch with smooth animation.

// Markup:
//   <switch checked="true" on-change="toggle_wifi">Wi-Fi</switch>
//   <switch disabled="true">Locked</switch>

const std = @import("std");
const ui = @import("ui");

pub fn switch_(opts: struct {
    checked: bool = false,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
    text: []const u8,
}, content: anytype) ui.Element {
    return ui.el(.switch, .{
        .checked = opts.checked,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
        .text = opts.text,
    }, content);
}

test "switch renders" {
    const el = switch_(.{ .text = "Wi-Fi", .checked = true }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .@"switch"), el.type);
}
