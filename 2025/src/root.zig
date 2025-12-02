const std = @import("std");
const Allocator = @import("std").mem.Allocator;
const ArrayList = @import("std").ArrayList;

pub const NoInputFileError = error {
    NoInputFileSpecified,
};

pub fn readInputFile(alloc: Allocator) !ArrayList([]u8) {
    const args = std.os.argv;

    if (args.len < 2) {
        return NoInputFileError.NoInputFileSpecified;
    }

    const filename = args[1];
    var pos: usize = 0;

    while (filename[pos] != 0) {
        pos += 1;
    }

    return readLines(filename[0..pos], alloc);
}

pub fn printPart1(part_1: anytype) !void {
    var buffer: [1024]u8 = undefined;
    var writer = std.fs.File.stdout().writer(&buffer);
    try std.Io.Writer.print(&writer.interface, "Part 1: {d}", .{part_1});
    try writer.interface.flush();
}

pub fn printDay(part_1: anytype, part_2: anytype) !void {
    var buffer: [1024]u8 = undefined;
    var writer = std.fs.File.stdout().writer(&buffer);
    try std.Io.Writer.print(&writer.interface, "Part 1: {d}\nPart 2: {d}", .{part_1, part_2});
    try writer.interface.flush();
}

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
        try std.testing.expectEqualStrings(ref, line);
    }
}
