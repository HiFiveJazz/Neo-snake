use std::{thread, time};

struct Display {
    row_1: String,
    row_2: String,
    row_3: String,
    row_4: String,
    row_5: String,
    row_6: String,
    row_7: String,
    row_8: String,
}

fn main() {
    let display_length: u8 = 8;
    
    let mut display = Display {
        row_1: "|          |".to_string(),
        row_2: "|          |".to_string(),
        row_3: "|          |".to_string(),
        row_4: "|          |".to_string(),
        row_5: "|          |".to_string(),
        row_6: "|          |".to_string(),
        row_7: "|          |".to_string(),
        row_8: "|          |".to_string(),
    };

    render(&display);
    display.row_1 = change_line(&display.row_1, 8, display_length);
    // println!("Test {}", display.row_1);
    render(&display);
    display.row_2 = change_line(&display.row_2, 8, display_length);
    render(&display);
    display.row_3 = change_line(&display.row_3, 8, display_length);
    render(&display);
    display.row_4 = change_line(&display.row_4, 8, display_length);
    render(&display);
    display.row_5 = change_line(&display.row_5, 8, display_length);
    render(&display);
    display.row_6 = change_line(&display.row_6, 8, display_length);
    render(&display);
    display.row_7 = change_line(&display.row_7, 8, display_length);
    render(&display);
    display.row_8 = change_line(&display.row_8, 8, display_length);
    render(&display);
}

fn render(d: &Display) {
    
    /* 
    TODO: 
    - Center the print so its center of the terminal  
    */
    
    print!("{}[2J", 27 as char); // Magically clears the terminal via escape code

    let rows = [
        &d.row_1,
        &d.row_2,
        &d.row_3,
        &d.row_4,
        &d.row_5,
        &d.row_6,
        &d.row_7,
        &d.row_8,
    ];

    for row in rows {
        println!("{}", row);
    }
    thread::sleep(time::Duration::from_secs(1)); //for debugging
}

fn change_line(row: &str, index: u8, display_length: u8) -> String{

    /* 
    TODO: 
    - Making the index static, currently index value depends on
    values that may be already present in the same row
    */

    if index > display_length {
        panic!("Progam panicked, cannot call index outside of range of the display.")
    }

    let mut count: u8 = 0;
    row.chars()
        .map(|c| {
            if c == ' ' {
                count += 1;
                if count == 2 {
                    return '■';
                }
            }
            c
        })
        .collect()
}
