const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mod = b.addModule("engine", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    mod.link_libc = true;
    
    // GLFW
    mod.linkSystemLibrary("glfw", .{});

    // WGPU
    mod.addIncludePath(b.path("vendor/wgpu/include"));
    mod.addLibraryPath(b.path("vendor/wgpu/lib"));
    mod.linkSystemLibrary("wgpu_native", .{
        .preferred_link_mode = .static
    });

    // Translate C
    const translate_c = b.addTranslateC(.{
        .root_source_file = b.path("src/c.h"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    translate_c.addIncludePath(b.path("vendor/wgpu/include"));

    const c_mod = translate_c.createModule();
    mod.addImport("c", c_mod);

    const lib_tests = b.addTest(.{ .root_module = mod });
    const test_step = b.step("test", "Run engine tests");
    test_step.dependOn(&b.addRunArtifact(lib_tests).step);
}
