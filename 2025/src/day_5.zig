const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const aoc_2025 = @import("aoc_2025");
const Range = aoc_2025.RangeInclusive(u64);

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);
    const ranges, const ids = try parseLines(alloc, lines.items);

    const part_1 = countFresh(ranges.items, ids.items);
    const part_2 = countTotalFresh(ranges.items);
    try aoc_2025.printDay(part_1, part_2);
}

fn parseLines(alloc: Allocator, lines: []const []const u8) !struct { ArrayList(Range), ArrayList(u64) } {
    var ranges = ArrayList(Range).empty;
    var ids = ArrayList(u64).empty;

    const start_of_ids = try ranges: for (lines, 0..) |line, i| {
        if (line.len == 0) {
            break :ranges i + 1;
        }

        try ranges.append(alloc, try Range.parse(line));
    } else {
        break :ranges std.zig.string_literal.ParseError.InvalidLiteral;
    };

    for (lines[start_of_ids..]) |line| {
        try ids.append(alloc, try std.fmt.parseInt(u64, line, 10));
    }

    return .{ ranges, ids };
}

test parseLines {
    const alloc = std.testing.allocator;

    const ref_ranges = [_]Range{
        Range{ .from = 3, .to = 5 },
        Range{ .from = 10, .to = 14 },
        Range{ .from = 16, .to = 20 },
        Range{ .from = 12, .to = 18 },
    };
    const ref_ids = [_]u64{
        1,
        5,
        8,
        11,
        17,
        32,
    };

    const input = [_][]const u8{
        "3-5",
        "10-14",
        "16-20",
        "12-18",
        "",
        "1",
        "5",
        "8",
        "11",
        "17",
        "32",
    };

    var ranges, var ids = try parseLines(alloc, &input);
    defer ranges.deinit(alloc);
    defer ids.deinit(alloc);

    try std.testing.expectEqualDeep(&ref_ranges, ranges.items);
    try std.testing.expectEqualDeep(&ref_ids, ids.items);
}

fn countFresh(ranges: []const Range, ids: []const u64) usize {
    var num_fresh: usize = 0;

    for (ids) |id| {
        if (isFresh(id, ranges)) {
            num_fresh += 1;
        }
    }

    return num_fresh;
}

fn isFresh(id: u64, ranges: []const Range) bool {
    for (ranges) |range| {
        if (range.contains(id)) {
            return true;
        }
    }

    return false;
}

test countFresh {
    const ranges = [_]Range{
        Range{ .from = 3, .to = 5 },
        Range{ .from = 10, .to = 14 },
        Range{ .from = 16, .to = 20 },
        Range{ .from = 12, .to = 18 },
    };
    const ids = [_]u64{
        1,
        5,
        8,
        11,
        17,
        32,
    };

    try std.testing.expectEqual(3, countFresh(&ranges, &ids));
}

fn countTotalFresh(ranges: []Range) u64 {
    var num_fresh: u64 = 0;

    mergeRanges(ranges);

    for (ranges) |range| {
        if (range.from <= range.to) {
            num_fresh += range.to + 1 - range.from;
        }
    }

    return num_fresh;
}

fn mergeRanges(ranges: []Range) void {
    for (0..ranges.len) |i| {
        for (i + 1..ranges.len) |j| {
            if (ranges[j].from <= ranges[i].from and ranges[i].to <= ranges[j].to) {
                ranges[i].from = 1;
                ranges[i].to = 0;
            } else if (ranges[i].from <= ranges[j].from and ranges[j].to <= ranges[i].to) {
                ranges[j].from = 1;
                ranges[j].to = 0;
            } else {
                if (ranges[i].from <= ranges[j].from and ranges[j].from <= ranges[i].to) {
                    ranges[i].to = ranges[j].from - 1;
                }

                if (ranges[i].from <= ranges[j].to and ranges[j].to <= ranges[i].to) {
                    ranges[i].from = ranges[j].to + 1;
                }
            }
        }
    }
}

test countTotalFresh {
    var ranges = [_]Range{
        Range{ .from = 3, .to = 5 },
        Range{ .from = 10, .to = 14 },
        Range{ .from = 16, .to = 20 },
        Range{ .from = 12, .to = 18 },
    };

    try std.testing.expectEqual(14, countTotalFresh(&ranges));
}

test "countTotalFresh with different ranges" {
    var ranges = [_]Range{
        Range{ .from = 3, .to = 5 },
        Range{ .from = 11, .to = 20 },
        Range{ .from = 18, .to = 24 },
        Range{ .from = 9, .to = 19 },
        Range{ .from = 24, .to = 30 },
    };

    try std.testing.expectEqual(25, countTotalFresh(&ranges));
}
