const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn Grid(comptime T: type) type {
    return struct {
        values: []T,
        width: usize,
        height: usize,

        pub fn init(alloc: Allocator, width: usize, height: usize) !Grid(T) {
            const values = try alloc.alloc(T, width * height);

            return Grid(T){
                .values = values,
                .width = width,
                .height = height,
            };
        }

        pub fn deinit(self: Grid(T), alloc: Allocator) void {
            alloc.free(self.values);
        }

        pub fn value(self: Grid(T), x: usize, y: usize) T {
            return self.values[y * self.width + x];
        }

        pub fn setValue(self: Grid(T), x: usize, y: usize, val: T) void {
            self.values[y * self.width + x] = val;
        }

        pub fn neighbor(self: Grid(T), x: usize, y: usize, xx: i32, yy: i32) ?T {
            if (xx == 0 and yy == 0) {
                return null;
            }

            const xxx = @as(i32, @intCast(x)) + xx;
            const yyy = @as(i32, @intCast(y)) + yy;

            if (0 <= xxx and xxx < self.width and 0 <= yyy and yyy < self.height) {
                return self.value(@intCast(xxx), @intCast(yyy));
            } else {
                return null;
            }
        }
    };
}

test "values" {
    var values = [_]u8{0} ** 9;
    const grid = Grid(u8){
        .values = &values,
        .width = 3,
        .height = 3,
    };

    for (0..grid.height) |y| {
        for (0..grid.width) |x| {
            try std.testing.expectEqual(0, grid.value(x, y));
        }
    }

    grid.setValue(0, 2, 9);
    grid.setValue(1, 1, 5);
    grid.setValue(2, 0, 8);

    try std.testing.expectEqual(9, grid.value(0, 2));
    try std.testing.expectEqual(5, grid.value(1, 1));
    try std.testing.expectEqual(8, grid.value(2, 0));
}

test "neighbor" {
    var values = [_]u8{
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    };
    const grid = Grid(u8){
        .values = &values,
        .width = 3,
        .height = 3,
    };

    try std.testing.expectEqual(1, grid.neighbor(0, 0, 1, 0));
    try std.testing.expectEqual(0, grid.neighbor(1, 0, -1, 0));
    try std.testing.expectEqual(2, grid.neighbor(1, 0, 1, 0));
    try std.testing.expectEqual(8, grid.neighbor(2, 1, 0, 1));

    try std.testing.expectEqual(null, grid.neighbor(0, 0, -1, 0));
    try std.testing.expectEqual(null, grid.neighbor(0, 0, 0, -1));
    try std.testing.expectEqual(null, grid.neighbor(2, 0, 1, 0));
    try std.testing.expectEqual(null, grid.neighbor(0, 2, 0, 1));

    try std.testing.expectEqual(null, grid.neighbor(0, 0, 0, 0));
}
