const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const aoc_2025 = @import("aoc_2025");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const solver = Solver{
        .alloc = alloc,
    };
    try aoc_2025.solve(alloc, solver);
}

const Op = enum {
    Add,
    Mul,
};

const Solver = struct {
    alloc: Allocator,

    pub fn parseInput(self: Solver, lines: []const []const u8) !struct { ArrayList(ArrayList(u64)), ArrayList(Op) } {
        var numbers = try ArrayList(ArrayList(u64)).initCapacity(self.alloc, lines.len - 1);

        for (lines[0 .. lines.len - 1]) |line| {
            try numbers.append(self.alloc, try parseNumbers(self.alloc, line));
        }

        return .{ numbers, try parseOps(self.alloc, lines[lines.len - 1]) };
    }

    pub fn part1(self: Solver, input: struct { ArrayList(ArrayList(u64)), ArrayList(Op) }) !u64 {
        _ = self;
        const numbers, const ops = input;

        return sumAnswers(numbers.items, ops.items);
    }

    pub fn part2(self: Solver, input: struct { ArrayList(ArrayList(u64)), ArrayList(Op) }) !void {
        _ = self;
        _ = input;
    }
};

fn parseNumbers(alloc: Allocator, line: []const u8) !ArrayList(u64) {
    var numbers = ArrayList(u64).empty;

    parseNumbersInputParam(alloc, line, &numbers) catch |err| {
        numbers.deinit(alloc);
        return err;
    };

    return numbers;
}

fn parseNumbersInputParam(alloc: Allocator, line: []const u8, numbers: *ArrayList(u64)) !void {
    var start: usize = 0;

    for (0..line.len) |i| {
        if (line[i] == ' ') {
            if (i > 0 and line[i - 1] != ' ') {
                const parsed = try std.fmt.parseInt(u64, line[start..i], 10);
                try numbers.append(alloc, parsed);
            }

            start = i + 1;
        } else if (i == line.len - 1) {
            const parsed = try std.fmt.parseInt(u64, line[start..], 10);
            try numbers.append(alloc, parsed);
        }
    }
}

fn parseOps(alloc: Allocator, line: []const u8) !ArrayList(Op) {
    var ops = ArrayList(Op).empty;

    parseOpsInputParam(alloc, line, &ops) catch |err| {
        ops.deinit(alloc);
        return err;
    };

    return ops;
}

fn parseOpsInputParam(alloc: Allocator, line: []const u8, ops: *ArrayList(Op)) !void {
    for (line) |c| {
        if (c == '*') {
            try ops.append(alloc, Op.Mul);
        } else if (c == '+') {
            try ops.append(alloc, Op.Add);
        }
    }
}

test "parseNumbers end with space" {
    const alloc = std.testing.allocator;

    const line = "123 328  51 64 ";

    const ref = [_]u64{ 123, 328, 51, 64 };

    var parsed = try parseNumbers(alloc, line);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

test "parseNumbers start with space" {
    const alloc = std.testing.allocator;

    const line = "  6 98  215 314";

    const ref = [_]u64{ 6, 98, 215, 314 };

    var parsed = try parseNumbers(alloc, line);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

test parseOps {
    const alloc = std.testing.allocator;

    const line = "*   +   *   +  ";

    const ref = [_]Op{ Op.Mul, Op.Add, Op.Mul, Op.Add };

    var parsed = try parseOps(alloc, line);
    defer parsed.deinit(alloc);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

fn sumAnswers(numbers: []const ArrayList(u64), ops: []const Op) usize {
    var sum: usize = 0;

    for (0..numbers[0].items.len) |i| {
        var value: usize = if (ops[i] == Op.Add) 0 else 1;

        for (0..numbers.len) |j| {
            if (ops[i] == Op.Add) {
                value += numbers[j].items[i];
            } else {
                value *= numbers[j].items[i];
            }
        }

        sum += value;
    }

    return sum;
}

test sumAnswers {
    var n1 = [_]u64{ 123, 328, 51, 64 };
    var n2 = [_]u64{ 45, 64, 387, 23 };
    var n3 = [_]u64{ 6, 98, 215, 314 };

    const numbers = [_]ArrayList(u64){
        ArrayList(u64){ .items = &n1 },
        ArrayList(u64){ .items = &n2 },
        ArrayList(u64){ .items = &n3 },
    };
    const ops = [_]Op{ Op.Mul, Op.Add, Op.Mul, Op.Add };

    try std.testing.expectEqual(4277556, sumAnswers(&numbers, &ops));
}
