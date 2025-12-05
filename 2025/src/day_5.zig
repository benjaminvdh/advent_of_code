const std = @import("std");
const aoc_2025 = @import("aoc_2025");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Range = struct {
    from: u64,
    to: u64,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const lines = try aoc_2025.readInputFile(alloc);
    const ranges, const ids = try parseLines(alloc, lines.items);

    const part_1 = countFresh(ranges.items, ids.items);
    try aoc_2025.printPart1(part_1);
}

fn parseLines(alloc: Allocator, lines: []const []const u8) !struct { ArrayList(Range), ArrayList(u64) } {
    var ranges = ArrayList(Range).empty;
    var ids = ArrayList(u64).empty;

    const start_of_ids = try ranges: for (lines, 0..) |line, i| {
        if (line.len == 0) {
            break :ranges i + 1;
        }

        const separator_index = try sep: for (line, 0..) |c, j| {
            if (c == '-') {
                break :sep j;
            }
        } else break :sep std.zig.string_literal.ParseError.InvalidLiteral;

        const start_string = line[0..separator_index];
        const range_start = try std.fmt.parseInt(u64, start_string, 0);

        const end_string = line[separator_index + 1..];
        const range_end = try std.fmt.parseInt(u64, end_string, 0);

        const range: Range = .{ .from = range_start, .to = range_end };
        try ranges.append(alloc, range);
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
    const ref_ids = [_]u64{ 1, 5, 8, 11, 17, 32, };

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
        if (range.from <= id and id <= range.to) {
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
    const ids = [_]u64{ 1, 5, 8, 11, 17, 32, };

    try std.testing.expectEqual(3, countFresh(&ranges, &ids));
}
