use crate::vga;

const FG: vga::Color = vga::Color::Black;
const SH: vga::Color = vga::Color::LightGray;
const BG: vga::Color = vga::Color::White;

pub fn write_string(x: usize, y: usize, s: &str) {
    vga::WRITER.lock().write_xy_color_string(x, y, FG, BG, s);
}

pub fn create(x1: usize, x2: usize, y1: usize, y2: usize) {
    draw(x1, x2, y1, y2);
}

pub fn create_left_top(height: usize, width: usize) {
    draw(2, height, 1, width);
}

pub fn create_center() {
    let x1 = 25 / 4;
    let x2 = 25 - (25 / 4);

    let y1 = 79 / 4;
    let y2 = 79 - (79 / 4);

    draw(x1, x2, y1, y2);
}

fn draw(x1: usize, x2: usize, y1: usize, y2: usize) {
    for x in x1 + 1..x2 + 1 {
        for y in y1 + 1..y2 + 1 {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, SH, b' ');
        }
    }

    for x in x1..x2 {
        for y in y1..y2 {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, BG, b' ');
        }
    }

    for x in x1 + 1..x2 - 1 {
        vga::WRITER.lock().write_xy_color_byte(x, y1, FG, BG, b'|');
        vga::WRITER
            .lock()
            .write_xy_color_byte(x, y2 - 1, FG, BG, b'|');
    }

    for y in y1 + 1..y2 - 1 {
        vga::WRITER.lock().write_xy_color_byte(x1, y, FG, BG, b'=');
        vga::WRITER
            .lock()
            .write_xy_color_byte(x2 - 1, y, FG, BG, b'=');
    }
}
