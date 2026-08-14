// maccn native-sdk: Slider component
// A draggable slider for selecting a value in a range.

// Markup:
//   <slider min="0" max="100" value="50" on-change="set_volume" />
//   <slider min="0" max="1" value="0.75" disabled="true" />

const std = @import("std");
const ui = @import("ui");

pub fn slider(opts: struct {
    min: f32 = 0,
    max: f32 = 100,
    value: f32 = 0,
    disabled: bool = false,
    on_change: ?[]const u8 = null,
}) ui.Element {
    return ui.el(.slider, .{
        .min = opts.min,
        .max = opts.max,
        .value = opts.value,
        .disabled = opts.disabled,
        .on_change = opts.on_change,
    }, .{});
}

test "slider renders" {
    const el = slider(.{ .min = 0, .max = 100, .value = 50 });
    try std.testing.expectEqual(@as(ui.ElementType, .slider), el.type);
}
