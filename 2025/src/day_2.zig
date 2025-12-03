const std = @import("std");
const aoc_2025 = @import("aoc_2025");

const Range = struct {
    from: usize,
    to: usize,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);

    const ranges = try parse(lines.items[0], alloc);

    const part_1 = countInvalid(false, ranges.items);
    const part_2 = countInvalid(true, ranges.items);
    try aoc_2025.printDay(part_1, part_2);
}

fn parse(line: []const u8, alloc: std.mem.Allocator) !std.array_list.Aligned(Range, null) {
    var ranges = std.array_list.Aligned(Range, null).empty;
    var first_start: usize = 0;
    var first_end: usize = 0;

    for (line, 0..) |c, i| {
        if (c == '-') {
            first_end = i;
        } else if (c == ',' or i == line.len - 1) {
            const start_string = line[first_start..first_end];
            const range_start = try std.fmt.parseInt(usize, start_string, 0);

            const end_index = if (c == ',') i else i + 1;
            const end_string = line[first_end + 1..end_index];
            const range_end = try std.fmt.parseInt(usize, end_string, 0);

            const range: Range = .{ .from = range_start, .to = range_end };
            try ranges.append(alloc, range);

            first_start = i + 1;
        }
    }

    return ranges;
}

test parse {
    const alloc = std.testing.allocator;

    const line = "123-456,78901234-89012345";

    const r1: Range = Range{ .from = 123, .to = 456 };
    const r2: Range = Range{ .from = 78901234, .to = 89012345 };
    const ref = [_]Range{ r1, r2 };
    var parsed = try parse(line, alloc);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

fn countInvalid(comptime at_least_two: bool, ranges: []const Range) usize {
    var sum_invalid: usize = 0;

    for (ranges) |range| {
        for (range.from..range.to + 1) |number| {
            if (isInvalid(at_least_two, number)) {
                sum_invalid += number;
            }
        }
    }

    return sum_invalid;
}

fn isInvalid(comptime at_least_two: bool, number: usize) bool {
    const l = std.math.log10_int(number);

    for (1..l + 1) |i| {
        const denominator = std.math.pow(usize, 10, i);
        const remainder = std.math.rem(usize, number, denominator) catch 0;

        if (remainder != 0 and std.math.log10_int(remainder) == i - 1) {
            var reproduced = remainder;

            if (at_least_two) {
                var j = i;
                while (j < l + 1) : (j += i) {
                    const factor = std.math.pow(usize, 10, j);
                    reproduced += remainder * factor;
                }
            } else {
                reproduced += remainder * denominator;
            }

            if (reproduced == number) {
                return true;
            }
        }
    }

    return false;
}

test "isInvalid 101" {
    try std.testing.expect(!isInvalid(false, 101));
}

test "isInvalid simple" {
    try std.testing.expect(isInvalid(false, 22));
}

test "isInvalid complex" {
    try std.testing.expect(isInvalid(false, 123123));
}

test "not isInvalid three times" {
    try std.testing.expect(!isInvalid(false, 222));
}

test "not isInvalid" {
    try std.testing.expect(!isInvalid(false, 1223));
}

test "not isInvalid long" {
    try std.testing.expect(!isInvalid(false, 1234512344));
}

test countInvalid {
    const ranges = [_]Range{
        Range{ .from = 11, .to = 22 },
        Range{ .from = 95, .to = 115 },
        Range{ .from = 998, .to = 1012 },
        Range{ .from = 1188511880, .to = 1188511890 },
        Range{ .from = 222220, .to = 222224 },
        Range{ .from = 1698522, .to = 1698528 },
        Range{ .from = 446443, .to = 446449 },
        Range{ .from = 38593856, .to = 38593862 },
        Range{ .from = 565653, .to = 565659 },
        Range{ .from = 824824821, .to = 824824827 },
        Range{ .from = 2121212118, .to = 2121212124 },
    };

    try std.testing.expectEqual(1227775554, countInvalid(false, &ranges));
}

test "countInvalidAtLeastTwo" {
    const ranges = [_]Range{
        Range{ .from = 11, .to = 22 },
        Range{ .from = 95, .to = 115 },
        Range{ .from = 998, .to = 1012 },
        Range{ .from = 1188511880, .to = 1188511890 },
        Range{ .from = 222220, .to = 222224 },
        Range{ .from = 1698522, .to = 1698528 },
        Range{ .from = 446443, .to = 446449 },
        Range{ .from = 38593856, .to = 38593862 },
        Range{ .from = 565653, .to = 565659 },
        Range{ .from = 824824821, .to = 824824827 },
        Range{ .from = 2121212118, .to = 2121212124 },
    };

    try std.testing.expectEqual(4174379265, countInvalid(true, &ranges));
}
