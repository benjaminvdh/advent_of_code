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

    const answer = rotate(instructions);
    try aoc_2025.printPart1(answer);
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
    try std.testing.expect(parseLine("L123") == -123);
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
    try std.testing.expect(result == 3);
}
