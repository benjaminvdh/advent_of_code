const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mod = b.addModule("aoc_2025", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });

    const mod_tests = b.addTest(.{
        .root_module = mod,
    });

    const run_mod_tests = b.addRunArtifact(mod_tests);
    const test_step = b.step("test_root", "Run tests of root");
    test_step.dependOn(&run_mod_tests.step);

    addDay(b, target, optimize, mod, "day_1");
}

fn addDay(b: *std.Build, target: ?std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode, mod: *std.Build.Module, comptime executable: []const u8) void {
    const path = "src/" ++ executable ++ ".zig";
    const exe = b.addExecutable(.{
        .name = executable,
        .root_module = b.createModule(.{
            .root_source_file = b.path(path),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "aoc_2025", .module = mod },
            },
        }),
    });

    b.installArtifact(exe);

    const run_step = b.step("run_" ++ executable, "Run " ++ executable);

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });

    const run_exe_tests = b.addRunArtifact(exe_tests);

    const test_step = b.step("test_" ++ executable, "Run tests of " ++ executable);
    test_step.dependOn(&run_exe_tests.step);
}
