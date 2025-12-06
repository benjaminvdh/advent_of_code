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

    pub fn parseInput(self: Solver, lines: []const []const u8) !struct { []const []const u8, ArrayList(Op) } {
        return .{ lines[0 .. lines.len - 1], try parseOps(self.alloc, lines[lines.len - 1]) };
    }

    pub fn part1(self: Solver, input: struct { []const []const u8, ArrayList(Op) }) !u64 {
        const number_lines, const ops = input;

        const numbers = try parseNumbers(true, self.alloc, number_lines, ops.items.len);

        return sumAnswers(numbers.items, ops.items);
    }

    pub fn part2(self: Solver, input: struct { []const []const u8, ArrayList(Op) }) !u64 {
        const number_lines, const ops = input;

        const numbers = try parseNumbers(false, self.alloc, number_lines, ops.items.len);

        return sumAnswers(numbers.items, ops.items);
    }
};

fn findNextStartColumn(number_lines: []const []const u8, start_column: usize) usize {
    for (start_column..number_lines[0].len) |i| {
        if (isSpaceColumn(number_lines, i)) {
            return i;
        }
    }

    return number_lines[0].len;
}

fn isSpaceColumn(number_lines: []const []const u8, column: usize) bool {
    for (0..number_lines.len) |j| {
        if (number_lines[j][column] != ' ') {
            return false;
        }
    }

    return true;
}

fn parseNumbers(comptime horizontal: bool, alloc: Allocator, number_lines: []const []const u8, num_ops: usize) !ArrayList(ArrayList(u64)) {
    var numbers = try ArrayList(ArrayList(u64)).initCapacity(alloc, num_ops);
    errdefer freeListOfList(alloc, &numbers);

    var start_column: usize = 0;

    while (start_column < number_lines[0].len) {
        const end_column = findNextStartColumn(number_lines, start_column);

        if (horizontal) {
            numbers.appendAssumeCapacity(try parseHorizontalNumbersBetweenColumns(alloc, number_lines, start_column, end_column));
        } else {
            numbers.appendAssumeCapacity(try parseVerticalNumbersBetweenColumns(alloc, number_lines, start_column, end_column));
        }

        start_column = end_column + 1;
    }

    return numbers;
}

fn parseHorizontalNumbersBetweenColumns(alloc: Allocator, number_lines: []const []const u8, start_column: usize, end_column: usize) !ArrayList(u64) {
    var numbers = try ArrayList(u64).initCapacity(alloc, number_lines.len);
    errdefer numbers.deinit(alloc);

    for (number_lines) |number_line| {
        var start_col: usize = start_column;
        while (number_line[start_col] == ' ') {
            start_col += 1;
        }

        var end_col: usize = start_col;
        while (end_col < end_column and number_line[end_col] != ' ') {
            end_col += 1;
        }

        const parsed = try std.fmt.parseInt(u64, number_line[start_col..end_col], 10);

        numbers.appendAssumeCapacity(parsed);
    }

    return numbers;
}

fn parseVerticalNumbersBetweenColumns(alloc: Allocator, number_lines: []const []const u8, start_column: usize, end_column: usize) !ArrayList(u64) {
    var numbers = try ArrayList(u64).initCapacity(alloc, end_column - start_column);
    errdefer numbers.deinit(alloc);

    var buffer = try ArrayList(u8).initCapacity(alloc, number_lines.len);
    defer buffer.deinit(alloc);

    for (start_column..end_column) |column| {
        buffer.shrinkRetainingCapacity(0);

        var row: usize = 0;
        while (number_lines[row][column] == ' ') {
            row += 1;
        }

        while (row < number_lines.len and number_lines[row][column] != ' ') {
            buffer.appendAssumeCapacity(number_lines[row][column]);
            row += 1;
        }

        try numbers.append(alloc, try std.fmt.parseInt(u64, buffer.items, 10));
    }

    return numbers;
}

fn freeListOfList(alloc: Allocator, list: *ArrayList(ArrayList(u64))) void {
    for (0..list.items.len) |i| {
        list.items[i].deinit(alloc);
    }

    list.deinit(alloc);
}

test "parseNumbers horizontal" {
    const alloc = std.testing.allocator;

    const lines = [_][]const u8{
        "123 328  51 64 ",
        "  6 98  215 314",
    };

    var n1 = [_]u64{ 123, 6 };
    var n2 = [_]u64{ 328, 98 };
    var n3 = [_]u64{ 51, 215 };
    var n4 = [_]u64{ 64, 314 };
    const ref = [_]ArrayList(u64){
        ArrayList(u64){ .items = &n1, .capacity = 2 },
        ArrayList(u64){ .items = &n2, .capacity = 2 },
        ArrayList(u64){ .items = &n3, .capacity = 2 },
        ArrayList(u64){ .items = &n4, .capacity = 2 },
    };

    var parsed = try parseNumbers(true, alloc, &lines, 4);
    defer freeListOfList(alloc, &parsed);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

test "parseNumbers vertical" {
    const alloc = std.testing.allocator;

    var n1 = [_]u64{ 1, 24, 356 };
    var n2 = [_]u64{ 369, 248, 8 };
    var n3 = [_]u64{ 32, 581, 175 };
    var n4 = [_]u64{ 623, 431, 4 };

    const ref = [_]ArrayList(u64){
        ArrayList(u64){ .items = &n1, .capacity = 3 },
        ArrayList(u64){ .items = &n2, .capacity = 3 },
        ArrayList(u64){ .items = &n3, .capacity = 3 },
        ArrayList(u64){ .items = &n4, .capacity = 3 },
    };

    const number_lines = [_][]const u8{
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
    };

    var parsed = try parseNumbers(false, alloc, &number_lines, 4);
    defer freeListOfList(alloc, &parsed);

    try std.testing.expectEqualDeep(&ref, parsed.items);
}

fn parseOps(alloc: Allocator, line: []const u8) !ArrayList(Op) {
    var ops = ArrayList(Op).empty;
    errdefer ops.deinit(alloc);

    for (line) |c| {
        if (c == '*') {
            try ops.append(alloc, Op.Mul);
        } else if (c == '+') {
            try ops.append(alloc, Op.Add);
        }
    }

    return ops;
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

    for (numbers, ops) |numbers_for_op, op| {
        var value: usize = if (op == Op.Add) 0 else 1;

        for (numbers_for_op.items) |number| {
            if (op == Op.Add) {
                value += number;
            } else {
                value *= number;
            }
        }

        sum += value;
    }

    return sum;
}

test sumAnswers {
    var n1 = [_]u64{ 123, 45, 6 };
    var n2 = [_]u64{ 328, 64, 98 };
    var n3 = [_]u64{ 51, 387, 215 };
    var n4 = [_]u64{ 64, 23, 314 };

    const numbers = [_]ArrayList(u64){
        ArrayList(u64){ .items = &n1 },
        ArrayList(u64){ .items = &n2 },
        ArrayList(u64){ .items = &n3 },
        ArrayList(u64){ .items = &n4 },
    };
    const ops = [_]Op{ Op.Mul, Op.Add, Op.Mul, Op.Add };

    try std.testing.expectEqual(4277556, sumAnswers(&numbers, &ops));
}
