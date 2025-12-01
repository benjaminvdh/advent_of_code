const std = @import("std");
const Allocator = @import("std").mem.Allocator;
const ArrayList = @import("std").ArrayList;

pub fn readLines(path: []const u8, alloc: Allocator) !ArrayList([]u8) {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    var buffer: [4096]u8 = undefined;
    var reader = std.fs.File.reader(file, &buffer);
    var contents = try std.Io.Reader.allocRemaining(&reader.interface, alloc, std.Io.Limit.unlimited);

    var prev: usize = 0;
    var lines = ArrayList([]u8).empty;

    for (contents, 0..) |c, i| {
        if (c == '\n') {
            try lines.append(alloc, contents[prev..i]);
            prev = i + 1;
        }
    }

    return lines;
}

test readLines {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const alloc = arena.allocator();

    const refs = [_][]const u8{"Line one", "The second line", "", "1234", "The end."};
    const output = try readLines("readLinesTestFile.txt", alloc);

    for (output.items, refs) |line, ref| {
        try std.testing.expect(std.mem.eql(u8, line, ref));
    }
}
