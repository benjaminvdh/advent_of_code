const std = @import("std");
const Allocator = std.mem.Allocator;
const aoc_2025 = @import("aoc_2025");
const Grid = aoc_2025.Grid(Tile);

const Tile = enum {
    Empty,
    Beam,
    Splitter,
};

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

    pub fn parseInput(self: Solver, lines: []const []const u8) !Grid {
        return parse(self.alloc, lines);
    }

    pub fn part1(self: Solver, input: Grid) !usize {
        _ = self;

        return countNumSplits(input);
    }

    pub fn part2(self: Solver, input: Grid) !void {
        _ = self;
        _ = input;
    }
};

fn parse(alloc: Allocator, lines: []const []const u8) !Grid {
    var grid = try Grid.init(alloc, lines[0].len, lines.len);

    for (lines, 0..) |line, y| {
        for (line, 0..) |c, x| {
            const value = switch (c) {
                'S' => Tile.Beam,
                '^' => Tile.Splitter,
                else => Tile.Empty,
            };

            grid.setValue(x, y, value);
        }
    }

    return grid;
}

test parse {
    const alloc = std.testing.allocator;

    const input = [_][]const u8{
        "..S..",
        ".....",
        "..^..",
        ".....",
    };

    const ref = [_]Tile{
        Tile.Empty, Tile.Empty, Tile.Beam,     Tile.Empty, Tile.Empty,
        Tile.Empty, Tile.Empty, Tile.Empty,    Tile.Empty, Tile.Empty,
        Tile.Empty, Tile.Empty, Tile.Splitter, Tile.Empty, Tile.Empty,
        Tile.Empty, Tile.Empty, Tile.Empty,    Tile.Empty, Tile.Empty,
    };

    const parsed = try parse(alloc, &input);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.values);
}

fn countNumSplits(grid: Grid) usize {
    var num_splits: usize = 0;

    for (0..grid.height - 1) |y| {
        for (0..grid.width) |x| {
            if (grid.value(x, y) == Tile.Beam) {
                if (grid.value(x, y + 1) == Tile.Empty) {
                    grid.setValue(x, y + 1, Tile.Beam);
                } else if (grid.value(x, y + 1) == Tile.Splitter) {
                    num_splits += 1;

                    if (x > 0 and grid.value(x - 1, y + 1) == Tile.Empty) {
                        grid.setValue(x - 1, y + 1, Tile.Beam);
                    }

                    if (x < grid.width - 1 and grid.value(x + 1, y + 1) == Tile.Empty) {
                        grid.setValue(x + 1, y + 1, Tile.Beam);
                    }
                }
            }
        }
    }

    return num_splits;
}

test countNumSplits {
    const alloc = std.testing.allocator;

    const input = [_][]const u8{
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    };

    const grid = try parse(alloc, &input);
    defer grid.deinit(alloc);

    try std.testing.expectEqual(21, countNumSplits(grid));
}
