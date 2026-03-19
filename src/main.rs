use std::{thread, time};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::io::{self, Read, Write};
use std::sync::mpsc::{channel, Receiver};
use fastrand;

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

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

unsafe extern "C" {
    fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
}

const TCSANOW: i32 = 0;
const ICANON: u32 = 0x0002;
const ECHO: u32 = 0x0008;

fn enable_raw_mode() -> Termios {
    let fd = std::io::stdin().as_raw_fd();
    let mut termios: Termios = unsafe { std::mem::zeroed() };

    unsafe {
        tcgetattr(fd, &mut termios);
    }

    let original = termios;

    // Disable canonical mode & echo
    termios.c_lflag &= !(ICANON | ECHO);

    unsafe {
        tcsetattr(fd, TCSANOW, &termios);
    }

    original
}

fn disable_raw_mode(original: Termios) {
    let fd = std::io::stdin().as_raw_fd();
    unsafe {
        tcsetattr(fd, TCSANOW, &original);
    }
}

fn read_key() -> u8 {
    let mut buf = [0u8; 1];
    unsafe {
        read(0, buf.as_mut_ptr(), 1);
    }
    buf[0]
}

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

fn apple_coordinates(terminal_cols: &u16, terminal_rows: &u16) -> (u16, u16) { // coordinates of apple, --> (x: u16, y: 16)
    let rand_rows= fastrand::u16(0..=*terminal_rows);
    let rand_cols= fastrand::u16(0..=*terminal_cols);
    (rand_rows,rand_cols)
    //TODO: Include logic to ensure apple cannot spawn on top of snake
}


fn get_input() -> Receiver<char> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        for byte in stdin.bytes() {
            if let Ok(b) = byte {
                let c = b as char;
                tx.send(c).unwrap();
            }
        }
    });

    rx
}

fn draw_apple(x: u16, y: u16) {
    print!("\x1B[{};{}H\x1B[31m■\x1B[0m", y, x);
}

fn main() {
    let (terminal_cols, terminal_rows) = terminal_size().expect("Could not get terminal size");
    let (mut apple_x, mut apple_y) = apple_coordinates(&terminal_cols, &terminal_rows);
    let input = get_input();
    let mut x = terminal_cols / 4;
    let mut y = terminal_rows / 2;
    clear_screen();
    loop {
        clear_screen();
        draw_apple(apple_x, apple_y);
        draw_block(x, y);

        io::stdout().flush().unwrap();
        if let Ok(key) = input.try_recv() {
            match key {
                'w' => y -= 1,
                's' => y += 1,
                'a' => x -= 1,
                'd' => x += 1,
                _ => {}
            }
        }

        // x += 1;
        // if x <= terminal_cols {
        //     x += 1;
        // }

        thread::sleep(time::Duration::from_millis(120));
        // if y <= terminal_rows {
        //     y += 1;
        // }
        thread::sleep(time::Duration::from_millis(120));
        if y == terminal_rows || x  == terminal_cols {
            break
        }
        if x == apple_x && y == apple_y {
            // increase length of snake by 1
            apple_coordinates(&terminal_cols, &terminal_rows); // regenerate coordinates for apple
        } 
    }
    clear_screen();
    println!("Game Over!");
}
