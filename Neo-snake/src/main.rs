struct Display<'a> {
    row_1: &'a str,
    row_2: &'a str,
    row_3: &'a str,
    row_4: &'a str,
    row_5: &'a str,
    row_6: &'a str,
    row_7: &'a str,
    row_8: &'a str,
}

fn main() {
    let display = Display {
        row_1: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_2: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_3: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_4: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_5: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_6: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_7: "[ ][ ][ ][ ][ ][ ][ ][ ]",
        row_8: "[ ][ ][ ][ ][ ][ ][ ][ ]",
    };
    render(display.row_1);
}

fn render(row: &str) {
    for _ in 0..8 {
        println!("{}",row);
    }
}

fn change_line(row: &str, index: u8){

}
