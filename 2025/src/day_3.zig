const std = @import("std");
const aoc_2025 = @import("aoc_2025");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);

    const part_1 = getTotalOutputJoltage(2, lines.items);
    const part_2 = getTotalOutputJoltage(12, lines.items);
    try aoc_2025.printDay(part_1, part_2);
}

fn getTotalOutputJoltage(comptime num_flips: u8, joltages: []const []const u8) u64 {
    var sum: u64 = 0;

    for (joltages) |joltage| {
        sum += getOutputJoltage(num_flips, joltage);
    }

    return sum;
}

fn getOutputJoltage(comptime num_flips: u8, joltage: []const u8) u64 {
    var max: u64 = 0;
    var start: usize = 0;

    for (0..num_flips) |i| {
        const biggest_digit = getBiggestDigit(joltage[start .. joltage.len - (num_flips - i - 1)], &start);
        max += std.math.pow(u64, 10, num_flips - i - 1) * biggest_digit;
    }

    return max;
}

fn getBiggestDigit(joltage: []const u8, start: *usize) u8 {
    var max: u8 = 0;
    var max_index: usize = 0;

    for (joltage, 0..) |digit, i| {
        const value = digit - '0';

        if (value > max) {
            max = value;
            max_index = i;
        }
    }

    start.* += max_index + 1;

    return max;
}

test "getOutputJoltage 987654321111111" {
    try std.testing.expectEqual(98, getOutputJoltage(2, "987654321111111"));
}

test "getOutputJoltage 811111111111119" {
    try std.testing.expectEqual(89, getOutputJoltage(2, "811111111111119"));
}

test "getOutputJoltage 234234234234278" {
    try std.testing.expectEqual(78, getOutputJoltage(2, "234234234234278"));
}

test "getOutputJoltage 818181911112111" {
    try std.testing.expectEqual(92, getOutputJoltage(2, "818181911112111"));
}

test getTotalOutputJoltage {
    const joltages = [_][]const u8{ "987654321111111", "811111111111119", "234234234234278", "818181911112111" };

    try std.testing.expectEqual(357, getTotalOutputJoltage(2, &joltages));
}

test "getTotalOutputJoltage 12" {
    const joltages = [_][]const u8{ "987654321111111", "811111111111119", "234234234234278", "818181911112111" };

    try std.testing.expectEqual(3121910778619, getTotalOutputJoltage(12, &joltages));
}
