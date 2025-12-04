const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const aoc_2025 = @import("aoc_2025");
const Grid = aoc_2025.Grid;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);
    const grid = try parseLines(alloc, lines.items);

    const part_1 = countAccessible(grid);
    try aoc_2025.printPart1(part_1);
}

fn parseLines(alloc: Allocator, lines: []const []const u8) !Grid(bool) {
    var grid = try Grid(bool).init(alloc, lines[0].len, lines.len);

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
        false, false, true, true, false, true, true, true, true, false,
        true, true, true, false, true, false, true, false, true, true,
        true, true, true, true, true, false, true, false, true, true,
        true, false, true, true, true, true, false, false, true, false,
        true, true, false, true, true, true, true, false, true, true,
        false, true, true, true, true, true, true, true, false, true,
        false, true, false, true, false, true, false, true, true, true,
        true, false, true, true, true, false, true, true, true, true,
        false, true, true, true, true, true, true, true, true, false,
        true, false, true, false, true, true, true, false, true, false,
    };
    const ref = Grid(bool){
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

fn countAccessible(grid: Grid(bool)) usize {
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

fn isAccessible(grid: Grid(bool), x: usize, y: usize) bool {
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

test countAccessible {
    var values = [_]bool{
        false, false, true, true, false, true, true, true, true, false,
        true, true, true, false, true, false, true, false, true, true,
        true, true, true, true, true, false, true, false, true, true,
        true, false, true, true, true, true, false, false, true, false,
        true, true, false, true, true, true, true, false, true, true,
        false, true, true, true, true, true, true, true, false, true,
        false, true, false, true, false, true, false, true, true, true,
        true, false, true, true, true, false, true, true, true, true,
        false, true, true, true, true, true, true, true, true, false,
        true, false, true, false, true, true, true, false, true, false,
    };
    const grid = Grid(bool){
        .values = &values,
        .width = 10,
        .height = 10,
    };

    try std.testing.expectEqual(13, countAccessible(grid));
}
