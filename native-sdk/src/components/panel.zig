// maccn native-sdk: Panel component
// The plain surface container — background, border, radius.

// Markup:
//   <panel width="300" padding="16">
//     <column gap="8">
//       <text>Title</text>
//       <text foreground="text_muted">Description</text>
//     </column>
//   </panel>

const std = @import("std");
const ui = @import("ui");

pub fn panel(opts: struct {
    width: ?f32 = null,
    height: ?f32 = null,
    padding: ?f32 = null,
    radius: ?f32 = null,
    on_press: ?[]const u8 = null,
}, content: anytype) ui.Element {
    return ui.panel(.{
        .width = opts.width,
        .height = opts.height,
        .padding = opts.padding,
        .radius = opts.radius,
        .on_press = opts.on_press,
    }, content);
}

test "panel renders" {
    const el = panel(.{ .width = 300, .padding = 16 }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .panel), el.type);
}
