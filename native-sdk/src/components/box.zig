// maccn native-sdk: Box component
// A grouped settings container with rounded corners and padding.

// Markup:
//   <box width="300" padding="16">
//     <column gap="8">
//       <text>Settings</text>
//       <text foreground="text_muted">Grouped controls</text>
//     </column>
//   </box>

const std = @import("std");
const ui = @import("ui");

pub fn box(opts: struct {
    width: ?f32 = null,
    height: ?f32 = null,
    padding: ?f32 = null,
}, content: anytype) ui.Element {
    return ui.panel(.{
        .width = opts.width,
        .height = opts.height,
        .padding = opts.padding orelse 16,
        .variant = .boxed,
    }, content);
}

test "box renders" {
    const el = box(.{ .width = 300 }, .{});
    try std.testing.expectEqual(@as(ui.ElementType, .panel), el.type);
}
