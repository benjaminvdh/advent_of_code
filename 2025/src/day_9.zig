const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const aoc_2025 = @import("aoc_2025");

const Coord = struct {
    x: i64,
    y: i64,
};

const Line = struct {
    from: Coord,
    to: Coord,

    pub fn init(from: Coord, to: Coord) Line {
        if (from.x == to.x) {
            if (from.y < to.y) {
                return .{ .from = from, .to = to };
            } else {
                return .{ .from = to, .to = from };
            }
        } else {
            if (from.x < to.x) {
                return .{ .from = from, .to = to };
            } else {
                return .{ .from = to, .to = from };
            }
        }
    }
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

    pub fn part2(self: Solver, input: anytype) !i64 {
        return findLargestRedAndGreenRectangle(self.alloc, input);
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

fn findLargestRedAndGreenRectangle(alloc: Allocator, coords: []const Coord) !i64 {
    var largest: i64 = 0;

    const horizontals, const verticals = try getLines(alloc, coords);
    defer alloc.free(horizontals);
    defer alloc.free(verticals);

    for (0..coords.len) |i| {
        for (i + 1..coords.len) |j| {
            const dx = coords[i].x - coords[j].x;
            const dy = coords[i].y - coords[j].y;

            const dx_abs = if (dx > 0) dx else -dx;
            const dy_abs = if (dy > 0) dy else -dy;

            const area = (dx_abs + 1) * (dy_abs + 1);

            if (area > largest and hasRedGreenOnly(coords, horizontals, verticals, i, j)) {
                largest = area;
            }
        }
    }

    return largest;
}

test findLargestRedAndGreenRectangle {
    const alloc = std.testing.allocator;

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

    try std.testing.expectEqual(24, findLargestRedAndGreenRectangle(alloc, &input));
}

fn getLines(alloc: Allocator, coords: []const Coord) !struct { []Line, []Line } {
    var horizontals = try ArrayList(Line).initCapacity(alloc, coords.len / 2);
    var verticals = try ArrayList(Line).initCapacity(alloc, coords.len / 2);

    for (0..coords.len) |i| {
        const from = coords[i];
        const to = coords[(i + 1) % coords.len];

        if (from.x == to.x) {
            verticals.appendAssumeCapacity(Line.init(from, to));
        } else {
            horizontals.appendAssumeCapacity(Line.init(from, to));
        }
    }

    std.sort.heap(Line, horizontals.items, {}, horizontalLessThan);
    std.sort.heap(Line, verticals.items, {}, verticalLessThan);

    return .{ horizontals.items, verticals.items };
}

fn horizontalLessThan(_: void, a: Line, b: Line) bool {
    if (a.from.y != b.from.y) {
        return a.from.y < b.from.y;
    }

    if (a.from.x != b.from.x) {
        return a.from.x < b.from.x;
    }

    return a.to.x < b.to.x;
}

fn verticalLessThan(_: void, a: Line, b: Line) bool {
    if (a.from.x != b.from.x) {
        return a.from.x < b.from.x;
    }

    if (a.from.y != b.from.y) {
        return a.from.y < b.from.y;
    }

    return a.to.y < b.to.y;
}

fn hasRedGreenOnly(coords: []const Coord, horizontals: []const Line, verticals: []const Line, i: usize, j: usize) bool {
    const a = coords[i];
    const b = coords[j];

    return pointInInterior(coords, a, j) and pointInInterior(coords, b, i) and !intersectsVertically(verticals, Line.init(
        .{ .x = a.x, .y = a.y },
        .{ .x = b.x, .y = a.y },
    ), a, b) and !intersectsHorizontally(horizontals, Line.init(
        .{ .x = b.x, .y = a.y },
        .{ .x = b.x, .y = b.y },
    ), a, b) and !intersectsVertically(verticals, Line.init(
        .{ .x = b.x, .y = b.y },
        .{ .x = a.x, .y = b.y },
    ), a, b) and !intersectsHorizontally(horizontals, Line.init(
        .{ .x = a.x, .y = b.y },
        .{ .x = a.x, .y = a.y },
    ), a, b);
}

fn pointInInterior(coords: []const Coord, target: Coord, i: usize) bool {
    const curr = coords[i];
    const prev = coords[if (i > 0) i - 1 else coords.len - 1];
    const next = coords[(i + 1) % coords.len];

    const prev_line = sub(curr, prev);
    const prev_normal = Coord{ .x = -prev_line.y, .y = prev_line.x };

    const next_line = sub(next, curr);
    const next_normal = Coord{ .x = -next_line.y, .y = next_line.x };

    const dot_prev = dot(sub(target, prev), prev_normal);
    const dot_next = dot(sub(target, curr), next_normal);

    if (dot(sub(next, prev), prev_normal) > 0) {
        return dot_prev > 0 and dot_next > 0;
    } else {
        return dot_prev > 0 or dot_next > 0;
    }
}

fn dot(a: Coord, b: Coord) i64 {
    return a.x * b.x + a.y * b.y;
}

fn sub(a: Coord, b: Coord) Coord {
    return .{ .x = a.x - b.x, .y = a.y - b.y };
}

fn intersectsVertically(lines: []const Line, side: Line, a: Coord, b: Coord) bool {
    for (std.sort.upperBound(Line, lines, side.from.x, compareX)..std.sort.upperBound(Line, lines, side.to.x, compareX)) |i| {
        const line = lines[i];

        if (line.from.y < side.from.y and side.from.y < line.to.y) {
            return true;
        }

        if (line.from.y == side.from.y and between(a.y, b.y, line.to.y)) {
            return true;
        }

        if (line.to.y == side.from.y and between(a.y, b.y, line.from.y)) {
            return true;
        }
    }

    return false;
}

fn compareX(x: i64, line: Line) std.math.Order {
    return std.math.order(x, line.from.x);
}

fn intersectsHorizontally(lines: []const Line, side: Line, a: Coord, b: Coord) bool {
    for (std.sort.upperBound(Line, lines, side.from.y, compareY)..std.sort.upperBound(Line, lines, side.to.y, compareY)) |i| {
        const line = lines[i];

        if (line.from.x < side.from.x and side.from.x < line.to.x) {
            return true;
        }

        if (line.from.x == side.from.x and between(a.x, b.x, line.to.x)) {
            return true;
        }

        if (line.to.x == side.from.x and between(a.x, b.x, line.from.x)) {
            return true;
        }
    }

    return false;
}

fn compareY(y: i64, line: Line) std.math.Order {
    return std.math.order(y, line.from.y);
}

fn between(start: i64, end: i64, val: i64) bool {
    return (start < val and val < end) or (end < val and val < start);
}
