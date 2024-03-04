const std = @import("std");

const debug = std.debug.print;
const expect = @import("std").testing.expect;

const Suit = enum {
    var clubs: u8 = 0;
    var spades: u8 = 0;
    var diamonds: u8 = 0;
    var hearts: u8 = 0;

    pub fn isClubs(self: Suit) bool {
        return self == Suit.clubs;
    }
};

var foo: i32 = 1;
pub fn main() !void {}

test "suspend with no resume" {
    var frame = async func();
    _ = frame;
    try expect(foo == 2);
}
fn func() void {
    foo += 1;
    suspend {}
    foo += 1;
}

test "if statement" {
    const a = true;
    var x: u16 = 0;

    if (a) {
        x += 1;
    } else {
        x += 2;
    }
    try expect(x == 1);
}

test "for loop" {
    const string = [_]u8{ 'a', 'b', 'c' };

    for (string, 0..) |char, index| {
        std.debug.print("{d} -- {d}\n", .{ index, char });
    }
}

test "defer" {
    var x: i16 = 5;
    {
        defer x += 2;
        try expect(x == 5);
    }

    try expect(x == 7);
}

const FileOpenError = error{
    AccessDenined,
    OutOfMemory,
    FileNotFound,
};

const AllocationError = error{OutOfMemory};

test "coerce error from a subset to a superset" {
    const err: FileOpenError = AllocationError.OutOfMemory;
    try expect(err == FileOpenError.OutOfMemory);
}
