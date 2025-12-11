use std::{thread, time};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::io::{self, Write};

#[repr(C)]
#[derive(Debug)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

unsafe extern "C" {
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

const TIOCGWINSZ: u64 = 0x5413;

fn terminal_size() -> Option<(u16, u16)> {
    let mut ws: Winsize = unsafe { mem::zeroed() };
    let fd = io::stdout().as_raw_fd();
    let res = unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) };

    if res == 0 {
        Some((ws.ws_col, ws.ws_row))
    } else {
        None
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn draw_block(x: u16, y: u16) {
    print!("\x1B[{};{}H■", y, x);
}

fn main() {
    let (terminal_cols, terminal_rows) = terminal_size().expect("Could not get terminal size");
    
    let mut x = terminal_cols / 4;
    let mut y = terminal_rows / 2;

    // loop {
    clear_screen();
    draw_block(100, 5);
    io::stdout().flush().unwrap();

    x += 1;
    if x >= terminal_cols {
        x = 1;
    }

    thread::sleep(time::Duration::from_millis(120));
    // }
}

