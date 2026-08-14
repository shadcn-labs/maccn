const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .wasi,
    });

    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addSharedLibrary(.{
        .name = "maccn-preview",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
        .single_threaded = true,
    });

    lib.rdynamic = true;
    lib.entry = .disabled;

    b.installArtifact(lib);

    const copy = b.addInstallArtifact(lib, .{
        .dest_dir = .{ .override = .{ .custom = "../../docs/public/native-sdk" } },
    });
    b.getInstallStep().dependOn(&copy.step);
}
