// maccn native-sdk: Glass Panel component
// A translucent panel with blur and vibrancy effects.

// Markup:
//   <glass-panel material="regular">
//     <column gap="6">
//       <text>Vibrant content</text>
//     </column>
//   </glass-panel>
//   <glass-panel material="clear" />

const std = @import("std");
const ui = @import("ui");

pub fn glassPanel(opts: struct {
    material: enum { regular, clear } = .regular,
    width: ?f32 = null,
    height: ?f32 = null,
    padding: ?f32 = null,
}, content: anytype) ui.Element {
    return ui.panel(.{
        .material = opts.material,
        .width = opts.width,
        .height = opts.height,
        .padding = opts.padding,
    }, content);
}

test "glass panel renders" {
    const el = glassPanel(.{ .material = .regular }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .panel), el.type);
}
