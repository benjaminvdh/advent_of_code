const std = @import("std");

pub fn RangeInclusive(comptime T: type) type {
    return struct {
        from: T,
        to: T,

        pub fn parse(line: []const u8) !RangeInclusive(T) {
            const separator_index = try sep: for (line, 0..) |c, i| {
                if (c == '-') {
                    break :sep i;
                }
            } else {
                break :sep std.zig.string_literal.ParseError.InvalidLiteral;
            };

            const from = try std.fmt.parseInt(T, line[0..separator_index], 10);
            const to = try std.fmt.parseInt(T, line[separator_index + 1..], 10);

            return RangeInclusive(T) { .from = from, .to = to, };
        }

        pub fn contains(self: RangeInclusive(T), value: T) bool {
            return self.from <= value and value <= self.to;
        }
    };
}

test "parse" {
    const line = "123-456";

    const ref = RangeInclusive(usize) { .from = 123, .to = 456, };

    try std.testing.expectEqualDeep(ref, try RangeInclusive(usize).parse(line));
}
