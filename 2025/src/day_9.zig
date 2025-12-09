const std = @import("std");
const Allocator = std.mem.Allocator;
const aoc_2025 = @import("aoc_2025");

const Coord = struct {
    x: i64,
    y: i64,
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

    pub fn parseInput(self: Solver, lines: []const []const u8) ![]Coord {
        var coords = try self.alloc.alloc(Coord, lines.len);
        errdefer self.alloc.free(coords);

        for (lines, 0..) |line, i| {
            if (std.ascii.indexOfIgnoreCase(line, ",")) |sep| {
                const x = try std.fmt.parseInt(i64, line[0..sep], 10);
                const y = try std.fmt.parseInt(i64, line[sep + 1 ..], 10);
                coords[i] = .{ .x = x, .y = y };
            } else {
                return error.ParseError;
            }
        }

        return coords;
    }

    pub fn part1(self: Solver, input: []const Coord) !i64 {
        _ = self;

        return findLargestRectangle(input);
    }

    pub fn part2(self: Solver, input: anytype) !void {
        _ = self;
        _ = input;
    }
};

fn findLargestRectangle(coords: []const Coord) i64 {
    var largest: i64 = 0;

    for (0..coords.len) |i| {
        for (i + 1..coords.len) |j| {
            const dx = coords[i].x - coords[j].x;
            const dy = coords[i].y - coords[j].y;

            const dx_abs = if (dx > 0) dx else -dx;
            const dy_abs = if (dy > 0) dy else -dy;

            const area = (dx_abs + 1) * (dy_abs + 1);

            if (area > largest) {
                largest = area;
            }
        }
    }

    return largest;
}

test findLargestRectangle {
    const input = [_]Coord{
        .{ .x = 7, .y = 1 },
        .{ .x = 11, .y = 1 },
        .{ .x = 11, .y = 7 },
        .{ .x = 9, .y = 7 },
        .{ .x = 9, .y = 5 },
        .{ .x = 2, .y = 5 },
        .{ .x = 2, .y = 3 },
        .{ .x = 7, .y = 3 },
    };

    try std.testing.expectEqual(50, findLargestRectangle(&input));
}
