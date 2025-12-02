const std = @import("std");
const aoc_2025 = @import("aoc_2025");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);

    const instructions = try alloc.alloc(i32, lines.items.len);

    for (lines.items, 0..) |line, i| {
        instructions[i] = parseLine(line);
    }

    const part_1 = rotate(instructions);
    const part_2 = rotateWithIntermediates(instructions);
    try aoc_2025.printDay(part_1, part_2);
}

fn parseLine(line: []const u8) i32 {
    var factor: i32 = 1;
    var value: i32 = 0;

    var it = std.mem.reverseIterator(line);

    while (it.next()) |c| {
        switch (c) {
            'L' => value *= -1,
            '0'...'9' => value += factor * (c - '0'),
            else => {},
        }

        factor *= 10;
    }

    return value;
}

test parseLine {
    try std.testing.expectEqual(-123, parseLine("L123"));
}

fn rotate(instructions: []const i32) u32 {
    var position: i32 = 50;
    var num_zeroes: u32 = 0;

    for (instructions) |instruction| {
        position += instruction;

        if (std.math.rem(i32, position, 100) catch 0 == 0) {
            num_zeroes += 1;
        }
    }

    return num_zeroes;
}

test rotate {
    const sequence = [_]i32{-68, -30, 48, -5, 60, -55, -1, -99, 14, -82};
    const result = rotate(&sequence);
    try std.testing.expectEqual(3, result);
}

fn rotateWithIntermediates(instructions: []const i32) i32 {
    var position: i32 = 50;
    var num_zeroes: i32 = 0;

    for (instructions) |instruction| {
        if (instruction < 0 and std.math.rem(i32, position, 100) catch 0 == 0) {
            num_zeroes -= 1;
        }

        const old = std.math.divFloor(i32, position, 100) catch 0;
        position += instruction;
        const new = std.math.divFloor(i32, position, 100) catch 0;

        if (old < new) {
            num_zeroes += new - old;
        } else {
            num_zeroes += old - new;
        }

        if (std.math.rem(i32, position, 100) catch 0 == 0 and instruction < 0) {
            num_zeroes += 1;
        }
    }

    return num_zeroes;
}

test rotateWithIntermediates {
    const sequence = [_]i32{-68, -30, 48, -5, 60, -55, -1, -99, 14, -82};
    const result = rotateWithIntermediates(&sequence);
    try std.testing.expectEqual(6, result);
}

test "rotateWithIntermediates ending on 0" {
    const sequence = [_]i32{23, -73, 4, -12, 12, -3};
    const result = rotateWithIntermediates(&sequence);
    try std.testing.expectEqual(3, result);
}

test "rotateWithIntermediates positive" {
    const sequence = [_]i32{-50, 10, -10, 10, -10};
    const result = rotateWithIntermediates(&sequence);
    try std.testing.expectEqual(3, result);
}

test "rotateWithIntermediates negative" {
    const sequence = [_]i32{-60, 10, -10, 10, -10};
    const result = rotateWithIntermediates(&sequence);
    try std.testing.expectEqual(3, result);
}
