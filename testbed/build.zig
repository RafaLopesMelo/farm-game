const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const engine_dep = b.dependency("engine", .{
        .target = target,
        .optimize = optimize,
    });
    mod.addImport("engine", engine_dep.module("engine"));

    const exe = b.addExecutable(.{
        .name = "testbed",
        .root_module = mod,
    });
    exe.use_lld = false;

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the testbed");
    run_step.dependOn(&run_cmd.step);
}
