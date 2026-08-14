// maccn native-sdk: Progress component
// A horizontal progress bar with determinate and indeterminate modes.

// Markup:
//   <progress value="0.6" />
//   <progress indeterminate="true" />
//   <progress value="0.35" size="sm" />

const std = @import("std");
const ui = @import("ui");

pub fn progress(opts: struct {
    value: f32 = 0,
    indeterminate: bool = false,
    size: enum { sm, default } = .default,
}) ui.Element {
    return ui.el(.progress, .{
        .value = opts.value,
        .indeterminate = opts.indeterminate,
        .size = opts.size,
    }, .{});
}

test "progress renders" {
    const el = progress(.{ .value = 0.6 });
    try std.testing.expectEqual(@as(ui.ElementType, .progress), el.type);
}
