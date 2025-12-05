const std = @import("std");
const Allocator = std.mem.Allocator;
const aoc_2025 = @import("aoc_2025");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const solver = Solver{
        .alloc = alloc,
    };
    try aoc_2025.solve(alloc, solver);
}

const Solver = struct {
    alloc: Allocator,

    pub fn parseInput(self: Solver, lines: []const []const u8) !void {
        _ = self;
        _ = lines;
    }

    pub fn part1(self: Solver, input: anytype) !void {
        _ = self;
        _ = input;
    }

    pub fn part2(self: Solver, input: anytype) !void {
        _ = self;
        _ = input;
    }
};
