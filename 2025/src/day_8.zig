const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const aoc_2025 = @import("aoc_2025");

const Junction = struct {
    x: i64,
    y: i64,
    z: i64,
    circuit: usize,
};

const Edge = struct {
    a: *Junction,
    b: *Junction,

    fn squaredLength(self: Edge) i64 {
        const dx = self.a.x - self.b.x;
        const dy = self.a.y - self.b.y;
        const dz = self.a.z - self.b.z;

        return dx * dx + dy * dy + dz * dz;
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
        return connectJunctions(1000, self.alloc, input.items);
    }

    pub fn part2(self: Solver, input: ArrayList(Junction)) !usize {
        return connectJunctions(std.math.maxInt(usize), self.alloc, input.items);
    }
};

fn parse(alloc: Allocator, lines: []const []const u8) !ArrayList(Junction) {
    var junctions = try ArrayList(Junction).initCapacity(alloc, lines.len);
    errdefer junctions.deinit(alloc);

    for (lines, 0..) |line, i| {
        if (std.ascii.indexOfIgnoreCasePos(line, 0, ",")) |split_xy| {
            if (std.ascii.indexOfIgnoreCasePos(line, split_xy + 1, ",")) |split_yz| {
                const x = try std.fmt.parseInt(i64, line[0..split_xy], 10);
                const y = try std.fmt.parseInt(i64, line[split_xy + 1 .. split_yz], 10);
                const z = try std.fmt.parseInt(i64, line[split_yz + 1 ..], 10);

                junctions.appendAssumeCapacity(.{ .x = x, .y = y, .z = z, .circuit = i });
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
        .{ .x = 162, .y = 817, .z = 812, .circuit = 0 },
        .{ .x = 57, .y = 618, .z = 57, .circuit = 1 },
        .{ .x = 906, .y = 360, .z = 560, .circuit = 2 },
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

fn connectJunctions(comptime iterations: usize, alloc: Allocator, junctions: []Junction) !usize {
    var edges = try getSortedEdges(alloc, junctions);
    defer edges.deinit(alloc);

    for (edges.items[0..if (iterations < edges.items.len) iterations else edges.items.len]) |edge| {
        if (edge.a.circuit != edge.b.circuit) {
            const from_circuit = edge.b.circuit;
            const to_circuit = edge.a.circuit;

            for (0..junctions.len) |i| {
                if (junctions[i].circuit == from_circuit) {
                    junctions[i].circuit = to_circuit;
                }
            }
        }

        if (allEqual(junctions)) {
            return @intCast(edge.a.x * edge.b.x);
        }
    }

    var groups = try alloc.alloc(usize, junctions.len);
    defer alloc.free(groups);

    for (0..groups.len) |i| {
        groups[i] = 0;
    }

    for (junctions) |j| {
        groups[j.circuit] += 1;
    }

    std.sort.heap(usize, groups, {}, std.sort.desc(usize));

    return groups[0] * groups[1] * groups[2];
}

fn allEqual(junctions: []const Junction) bool {
    for (junctions) |junction| {
        if (junction.circuit != junctions[0].circuit) {
            return false;
        }
    }

    return true;
}

fn getSortedEdges(alloc: Allocator, junctions: []Junction) !ArrayList(Edge) {
    const num: usize = junctions.len * (junctions.len + 1) / 2;
    var edges = try ArrayList(Edge).initCapacity(alloc, num);

    for (0..junctions.len) |i| {
        for (i + 1..junctions.len) |j| {
            edges.appendAssumeCapacity(.{ .a = &junctions[i], .b = &junctions[j] });
        }
    }

    std.sort.heap(Edge, edges.items, {}, compareEdgeLength);

    return edges;
}

fn compareEdgeLength(_: void, lhs: Edge, rhs: Edge) bool {
    return lhs.squaredLength() < rhs.squaredLength();
}

test connectJunctions {
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

    const size = connectJunctions(10, alloc, parsed.items);
    try std.testing.expectEqual(40, size);
}

test "connectAllJunctions" {
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

    const coords = connectJunctions(std.math.maxInt(usize), alloc, parsed.items);
    try std.testing.expectEqual(25272, coords);
}
