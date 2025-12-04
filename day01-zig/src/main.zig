const std = @import("std");

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const arg1 = args.next();

    var part: [:0] const u8 = undefined;

    if (arg1) |a| {
        part = a;
    } else {
        part = "1";
    }

    if (std.mem.eql(u8, part, "1")) {
        try part1();
    } else {
        try part2();
    }
}

fn part1() !void {
    var buf: [20]u8 = undefined;
    var stdin = std.fs.File.stdin();
    var stdin_reader = stdin.reader(&buf);
    const empty: []u8 = &.{};
    const reader: *std.Io.Reader = &stdin_reader.interface;

    var dial: i32 = 50;
    var count: i32 = 0;

    var line = reader.takeDelimiterExclusive('\n') catch empty;
    while (line.len > 0) {
        const numStr = line[1..];
        var delta = try std.fmt.parseInt(i32, numStr, 10);
        if (line[0] == 'L') {
            delta *= -1;
        }
        
        dial = @mod(dial + delta, 100);
        if (dial == 0) {
            count += 1;
        }

        reader.toss(1);
        line = reader.takeDelimiterExclusive('\n') catch empty;
    }
    std.debug.print("{}\n", .{count});
}

fn part2() !void {
    var buf: [20]u8 = undefined;
    var stdin = std.fs.File.stdin();
    var stdin_reader = stdin.reader(&buf);
    const empty: []u8 = &.{};
    const reader: *std.Io.Reader = &stdin_reader.interface;

    var dial: i32 = 50;
    var count: i32 = 0;

    var line = reader.takeDelimiterExclusive('\n') catch empty;
    while (line.len > 0) {
        const numStr = line[1..];
        var delta = try std.fmt.parseInt(i32, numStr, 10);
        if (line[0] == 'L') {
            delta *= -1;
        }

        const newDial = dial + delta;
        if (newDial == 0) {
            count += 1;
        } else if (newDial < 0 and dial > 0) {
            count += @divFloor(newDial, (-100)) + 1;
        } else if (newDial < 0 and dial == 0) {
            count += @divFloor(newDial ,-100);
        } else {
            count += @divFloor(newDial, 100);
        }
        
        dial = @mod(newDial, 100);

        reader.toss(1);
        line = reader.takeDelimiterExclusive('\n') catch empty;
    }
    std.debug.print("{}\n", .{count});
}
