package main

import "core:fmt"
import "core:os"
import "core:bufio"
import "core:strconv"
import "core:strings"

main :: proc() {
    if (len(os.args) < 2 || os.args[1] == "1") {
        part1()
    } else {
        part2()
    }
}

part1 :: proc() {
    r: bufio.Reader
    buffer: [20]byte
    bufio.reader_init_with_buf(&r, os.stream_from_handle(os.stdin), buffer[:]);
    defer bufio.reader_destroy(&r);

    dial: int = 50;
    count: int = 0;

    for {
        line, err := bufio.reader_read_string(&r, '\n');
        if err != nil {
            break
        }
        defer delete(line);

        delta, ok := strconv.parse_int(strings.trim_space(line[1:]));
        if !ok {
            break
        }

        if line[0] == 'L' {
            delta *= -1
        }

        dial = (((dial + delta) % 100) + 100) % 100

        if dial == 0 {
            count += 1
        }
    }

    fmt.println(count);
}

part2 :: proc() {
    r: bufio.Reader
    buffer: [20]byte
    bufio.reader_init_with_buf(&r, os.stream_from_handle(os.stdin), buffer[:]);
    defer bufio.reader_destroy(&r);

    dial: int = 50;
    count: int = 0;

    for {
        line, err := bufio.reader_read_string(&r, '\n');
        if err != nil {
            break
        }
        defer delete(line);

        delta, ok := strconv.parse_int(strings.trim_space(line[1:]));
        if !ok {
            break
        }

        if line[0] == 'L' {
            delta *= -1
        }

        new_dial := dial + delta

        if new_dial == 0 {
            count += 1
        } else if new_dial < 0 && dial == 0 {
            count += new_dial / (-100);
        } else if new_dial < 0 && dial > 0 {
            count += new_dial / (-100) + 1;
        } else {
            count += new_dial / 100;
        }

        dial = (((new_dial) % 100) + 100) % 100
    }

    fmt.println(count);
}
