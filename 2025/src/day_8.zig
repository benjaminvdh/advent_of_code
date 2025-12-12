const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap(Junction, void, Hasher, 50);
const aoc_2025 = @import("aoc_2025");

const Junction = struct {
    x: f64,
    y: f64,
    z: f64,

    fn eq(self: Junction, other: Junction) bool {
        return self.x == other.x and self.y == other.y and self.z == other.z;
    }
};

const Edge = struct {
    a: Junction,
    b: Junction,

    fn length(self: Edge) f64 {
        const dx = self.a.x - self.b.x;
        const dy = self.a.y - self.b.y;
        const dz = self.a.z - self.b.z;

        return std.math.sqrt(dx * dx + dy * dy + dz * dz);
    }
};

const Hasher = struct {
    pub fn hash(_: Hasher, key: Junction) u64 {
        return @as(u64, @bitCast(key.x)) ^ @as(u64, @bitCast(key.y)) ^ @as(u64, @bitCast(key.z));
    }

    pub fn eql(_: Hasher, a: Junction, b: Junction) bool {
        return a.eq(b);
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

    pub fn parseInput(self: Solver, lines: []const []const u8) !ArrayList(Junction) {
        return parse(self.alloc, lines);
    }

    pub fn part1(self: Solver, input: ArrayList(Junction)) !usize {
        return getSizeOfLargestGroups(1000, self.alloc, input.items);
    }

    pub fn part2(_: Solver, _: ArrayList(Junction)) !void {}
};

fn parse(alloc: Allocator, lines: []const []const u8) !ArrayList(Junction) {
    var junctions = try ArrayList(Junction).initCapacity(alloc, lines.len);
    errdefer junctions.deinit(alloc);

    for (lines) |line| {
        if (std.ascii.indexOfIgnoreCasePos(line, 0, ",")) |split_xy| {
            if (std.ascii.indexOfIgnoreCasePos(line, split_xy + 1, ",")) |split_yz| {
                const x = try std.fmt.parseFloat(f64, line[0..split_xy]);
                const y = try std.fmt.parseFloat(f64, line[split_xy + 1 .. split_yz]);
                const z = try std.fmt.parseFloat(f64, line[split_yz + 1 ..]);

                junctions.appendAssumeCapacity(.{ .x = x, .y = y, .z = z });
            } else {
                return error.ParseError;
            }
        } else {
            return error.ParseError;
        }
    }

    return junctions;
}

test parse {
    const alloc = std.testing.allocator;

    const ref = [_]Junction{
        .{ .x = 162, .y = 817, .z = 812 },
        .{ .x = 57, .y = 618, .z = 57 },
        .{ .x = 906, .y = 360, .z = 560 },
    };

    const input = [_][]const u8{
        "162,817,812",
        "57,618,57",
        "906,360,560",
    };

    var parsed = try parse(alloc, &input);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

fn getSizeOfLargestGroups(comptime iterations: usize, alloc: Allocator, junctions: []Junction) !usize {
    var edges = try getSortedEdges(iterations, alloc, junctions);
    defer edges.deinit(alloc);

    return connect(alloc, &edges);
}

test getSizeOfLargestGroups {
    const alloc = std.testing.allocator;

    const input = [_][]const u8{
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    };

    var parsed = try parse(alloc, &input);
    defer parsed.deinit(alloc);

    const size = getSizeOfLargestGroups(10, alloc, parsed.items);
    try std.testing.expectEqual(40, size);
}

fn getSortedEdges(comptime iterations: usize, alloc: Allocator, coords: []Junction) !ArrayList(Edge) {
    const num: usize = coords.len * (coords.len + 1) / 2;
    var edges = try ArrayList(Edge).initCapacity(alloc, num);

    for (0..coords.len) |i| {
        for (i + 1..coords.len) |j| {
            edges.appendAssumeCapacity(.{ .a = coords[i], .b = coords[j] });
        }
    }

    std.sort.heap(Edge, edges.items, {}, compareEdgeLength);

    edges.shrinkAndFree(alloc, iterations);

    return edges;
}

fn compareEdgeLength(_: void, lhs: Edge, rhs: Edge) bool {
    return lhs.length() < rhs.length();
}

fn connect(alloc: Allocator, edges: *ArrayList(Edge)) !usize {
    var biggest_1: usize = 0;
    var biggest_2: usize = 0;
    var biggest_3: usize = 0;

    while (edges.items.len > 0) {
        var network = HashMap.init(alloc);
        defer network.deinit();

        try fillNetwork(alloc, edges, &network);

        const size = network.count();

        if (size > biggest_1) {
            biggest_3 = biggest_2;
            biggest_2 = biggest_1;
            biggest_1 = size;
        } else if (size > biggest_2) {
            biggest_3 = biggest_2;
            biggest_2 = size;
        } else if (size > biggest_3) {
            biggest_3 = size;
        }
    }

    return biggest_1 * biggest_2 * biggest_3;
}

fn fillNetwork(_: Allocator, edges: *ArrayList(Edge), network: *HashMap) !void {
    if (edges.pop()) |edge| {
        try network.put(edge.a, {});
        try network.put(edge.b, {});

        try extendNetwork(edges, network, edge.a);
        try extendNetwork(edges, network, edge.b);
    }
}

fn extendNetwork(edges: *ArrayList(Edge), network: *HashMap, from_node: Junction) !void {
    var changed = true;

    while (changed) {
        changed = false;

        for (0..edges.items.len) |i| {
            const e = edges.items[i];

            if (e.a.eq(from_node) or e.b.eq(from_node)) {
                _ = edges.orderedRemove(i);
                changed = true;

                if (!network.contains(e.a)) {
                    try network.put(e.a, {});
                    try extendNetwork(edges, network, e.a);
                }

                if (!network.contains(e.b)) {
                    try network.put(e.b, {});
                    try extendNetwork(edges, network, e.b);
                }

                break;
            }
        }
    }
}
