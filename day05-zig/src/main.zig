const std = @import("std");

var stdout_buffer: [1024]u8 = undefined;
var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
const stdout = &stdout_writer.interface;

var stdin_buffer: [40]u8 = undefined;
var stdin_reader = std.fs.File.stdin().reader(&stdin_buffer);
var reader: *std.Io.Reader = &stdin_reader.interface;

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const part = args.next() orelse "1";

    if (std.mem.eql(u8, part, "1")) {
        try part1();
    } else {
        try part2();
    }
}

fn part1() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var intervals = try std.ArrayList(struct {u64, u64})
        .initCapacity(allocator, 100); 
    defer intervals.deinit(allocator);

    var line = try reader.takeDelimiterExclusive('\n');
    while (line.len > 0) {
        var it = std.mem.splitSequence(u8, line, "-");
        const startStr = it.first();
        const endStr = it.next().?;

        const start = try std.fmt.parseInt(u64, startStr, 10);
        const end = try std.fmt.parseInt(u64, endStr, 10);
        
        try intervals.append(allocator, .{start, end});

        _ = reader.toss(1);
        line = reader.takeDelimiterExclusive('\n') catch break;
    }

    var count: u64 = 0;
    _ = reader.toss(1);
    line = try reader.takeDelimiterExclusive('\n');
    while (line.len > 0) {
        const num = try std.fmt.parseInt(u64, line, 10);

        for (intervals.items) |i| {
            if (i[0] <= num and num <= i[1]) {
                count += 1;
                break;
            }
        }

        _ = reader.toss(1);
        line = reader.takeDelimiterExclusive('\n') catch break;
    }

    try stdout.print("{}\n", .{count});
    try stdout.flush();
}

fn part2() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var intervals = try std.ArrayList(struct {u64, u64})
        .initCapacity(allocator, 100); 
    defer intervals.deinit(allocator);

    var line = try reader.takeDelimiterExclusive('\n');
    while (line.len > 0) {
        var it = std.mem.splitSequence(u8, line, "-");
        const startStr = it.first();
        const endStr = it.next().?;

        const start = try std.fmt.parseInt(u64, startStr, 10);
        const end = try std.fmt.parseInt(u64, endStr, 10);
        
        try intervals.append(allocator, .{start, end});

        _ = reader.toss(1);
        line = reader.takeDelimiterExclusive('\n') catch break;
    }

    std.mem.sort(struct {u64, u64}, intervals.items, {}, compare);
    var count: u64 = 0;

    var lastInterval = intervals.items[0];
    for (intervals.items) |i| {
        if (i[0] == lastInterval[0]) {
            lastInterval[1] = i[1];
        } else if (i[0] <= lastInterval[1] and i[1] >= lastInterval[1]) {
            lastInterval[1] = i[1];
        } else if (i[0] > lastInterval[1]) {
            count += lastInterval[1] - lastInterval[0] + 1;
            lastInterval = i;
        }
    }
    count += lastInterval[1] - lastInterval[0] + 1;

    try stdout.print("{}\n", .{count});
    try stdout.flush();
}

fn compare (_: void, a: struct {u64, u64}, b: struct {u64, u64}) bool {
    return a[0] < b[0] or (a[0] == b[0] and a[1] < b[1]);
}

