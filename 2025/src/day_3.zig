const std = @import("std");
const aoc_2025 = @import("aoc_2025");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);

    const part_1 = getTotalOutputJoltage(lines.items);
    try aoc_2025.printPart1(part_1);
}

fn getTotalOutputJoltage(joltages: []const []const u8) u64 {
    var sum: u64 = 0;

    for (joltages) |joltage| {
        sum += getOutputJoltage(joltage);
    }
    
    return sum;
}

fn getOutputJoltage(joltage: []const u8) u8 {
    const num_digits = joltage.len;

    var max: u8 = 0;

    for (0..num_digits) |i| {
        const second_digit = getNthDigit(joltage, i);

        for (i + 1..num_digits) |j| {
            const first_digit = getNthDigit(joltage, j);

            const sum = second_digit + 10 * first_digit;

            if (sum > max) {
                max = sum;
            }
        }
    }

    return max;
}

fn getNthDigit(joltage: []const u8, index: usize) u8 {
    const i = joltage.len - index - 1;
    return std.fmt.parseInt(u8, joltage[i..i + 1], 10) catch 0;
}

test "getOutputJoltage 987654321111111" {
    try std.testing.expectEqual(98, getOutputJoltage("987654321111111"));
}

test "getOutputJoltage 811111111111119" {
    try std.testing.expectEqual(89, getOutputJoltage("811111111111119"));
}

test "getOutputJoltage 234234234234278" {
    try std.testing.expectEqual(78, getOutputJoltage("234234234234278"));
}

test "getOutputJoltage 818181911112111" {
    try std.testing.expectEqual(92, getOutputJoltage("818181911112111"));
}

test getTotalOutputJoltage {
    const joltages = [_][]const u8{ "987654321111111", "811111111111119", "234234234234278", "818181911112111" };

    try std.testing.expectEqual(357, getTotalOutputJoltage(&joltages));
}
