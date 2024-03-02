const std = @import("std");
const fs = std.fs;
const io = std.io;
const debug = std.debug.print;
const ascii = std.ascii;

const path = "sorted_ints.txt";

pub fn main() !void {
    const file = try fs.cwd().openFile(path, .{});

    const target: u8 = 5;

    const stat = file.stat() catch |err| {
        std.debug.panic("unable to read file stats: {}\n", .{err});
    };

    var lo: u64 = 0;
    var hi = stat.size - 1;
    var mid: u64 = 0;

    while (lo <= hi) {
        mid = lo + (hi - lo) / 2;

        file.seekTo(mid - 1) catch |err| {
            std.debug.panic("unable to seek to offset {} in file: {}\n", .{ mid, err });
        };

        const byte = readByte(file) catch |err| switch (err) {
            error.EndOfStream => return,
            else => std.debug.panic("unable to read byte: {}\n", .{err}),
        };

        // new line byte which is last byte before EOF
        if (byte == 10) {
            return;
        }

        const tmp: u8 = byte - 48;

        if (tmp == target) {
            debug("found target: {} number in file\n", .{target});
            return;
        }

        if (tmp < target) {
            lo = mid + 1;
        } else if (tmp > target) {
            hi = mid - 1;
        }
    }

    debug("Target {d} was not found in file {s}\n", .{ target, path });
}

fn readByte(file: fs.File) !u8 {
    return file.reader().readByte();
}

fn printByte(b: u8) void {
    const fake = [1]u8{b};
    debug("ASCII: {s} -- Byte: {}\n", .{ fake, b });
}
