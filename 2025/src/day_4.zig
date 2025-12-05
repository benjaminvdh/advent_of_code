const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const aoc_2025 = @import("aoc_2025");
const Grid = aoc_2025.Grid(bool);

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
        return parseLines(self.alloc, lines);
    }

    pub fn part1(self: Solver, input: Grid) !usize {
        _ = self;
        return countAccessible(input);
    }

    pub fn part2(self: Solver, input: Grid) !usize {
        const backbuffer = try Grid.init(self.alloc, input.width, input.height);
        return countRemovable(input, backbuffer);
    }
};

fn parseLines(alloc: Allocator, lines: []const []const u8) !Grid {
    var grid = try Grid.init(alloc, lines[0].len, lines.len);

    for (lines, 0..) |line, y| {
        for (line, 0..) |c, x| {
            grid.setValue(x, y, c == '@');
        }
    }

    return grid;
}

test parseLines {
    const alloc = std.testing.allocator;

    var ref_values = [_]bool{
        false, false, true,  true,  false, true,  true,  true,  true,  false,
        true,  true,  true,  false, true,  false, true,  false, true,  true,
        true,  true,  true,  true,  true,  false, true,  false, true,  true,
        true,  false, true,  true,  true,  true,  false, false, true,  false,
        true,  true,  false, true,  true,  true,  true,  false, true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  false, true,
        false, true,  false, true,  false, true,  false, true,  true,  true,
        true,  false, true,  true,  true,  false, true,  true,  true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  true,  false,
        true,  false, true,  false, true,  true,  true,  false, true,  false,
    };
    const ref = Grid{
        .values = &ref_values,
        .width = 10,
        .height = 10,
    };

    const lines = [_][]const u8{
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    };
    const parsed = try parseLines(alloc, &lines);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(ref, parsed);
}

fn countAccessible(grid: Grid) usize {
    var accessible: usize = 0;

    for (0..grid.height) |y| {
        for (0..grid.width) |x| {
            if (isAccessible(grid, x, y)) {
                accessible += 1;
            }
        }
    }

    return accessible;
}

fn isAccessible(grid: Grid, x: usize, y: usize) bool {
    if (!grid.value(x, y)) {
        return false;
    }

    var neighboring_papers: usize = 0;
    var yy: i32 = -1;

    while (yy < 2) : (yy += 1) {
        var xx: i32 = -1;

        while (xx < 2) : (xx += 1) {
            if (grid.neighbor(x, y, xx, yy)) |neighbor| {
                if (neighbor) {
                    neighboring_papers += 1;
                }
            }
        }
    }

    return neighboring_papers < 4;
}

fn countRemovable(grid: Grid, backbuffer: Grid) usize {
    var removable: usize = 0;
    var removable_this_time: usize = 1;

    while (removable_this_time != 0) {
        removable_this_time = 0;

        for (0..grid.height) |y| {
            for (0..grid.width) |x| {
                if (isAccessible(grid, x, y)) {
                    backbuffer.setValue(x, y, false);
                    removable_this_time += 1;
                    removable += 1;
                } else if (grid.value(x, y)) {
                    backbuffer.setValue(x, y, true);
                }
            }
        }

        for (0..grid.height) |y| {
            for (0..grid.width) |x| {
                grid.setValue(x, y, backbuffer.value(x, y));
            }
        }
    }

    return removable;
}

test countAccessible {
    var values = [_]bool{
        false, false, true,  true,  false, true,  true,  true,  true,  false,
        true,  true,  true,  false, true,  false, true,  false, true,  true,
        true,  true,  true,  true,  true,  false, true,  false, true,  true,
        true,  false, true,  true,  true,  true,  false, false, true,  false,
        true,  true,  false, true,  true,  true,  true,  false, true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  false, true,
        false, true,  false, true,  false, true,  false, true,  true,  true,
        true,  false, true,  true,  true,  false, true,  true,  true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  true,  false,
        true,  false, true,  false, true,  true,  true,  false, true,  false,
    };
    const grid = Grid{
        .values = &values,
        .width = 10,
        .height = 10,
    };

    try std.testing.expectEqual(13, countAccessible(grid));
}

test countRemovable {
    var values = [_]bool{
        false, false, true,  true,  false, true,  true,  true,  true,  false,
        true,  true,  true,  false, true,  false, true,  false, true,  true,
        true,  true,  true,  true,  true,  false, true,  false, true,  true,
        true,  false, true,  true,  true,  true,  false, false, true,  false,
        true,  true,  false, true,  true,  true,  true,  false, true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  false, true,
        false, true,  false, true,  false, true,  false, true,  true,  true,
        true,  false, true,  true,  true,  false, true,  true,  true,  true,
        false, true,  true,  true,  true,  true,  true,  true,  true,  false,
        true,  false, true,  false, true,  true,  true,  false, true,  false,
    };
    const grid = Grid{
        .values = &values,
        .width = 10,
        .height = 10,
    };

    var backbuffer_values: [100]bool = undefined;
    const backbuffer = Grid{
        .values = &backbuffer_values,
        .width = 10,
        .height = 10,
    };

    try std.testing.expectEqual(43, countRemovable(grid, backbuffer));
}
