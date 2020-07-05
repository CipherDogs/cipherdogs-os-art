use crate::vga;

const FG: vga::Color = vga::Color::Black;
const SH: vga::Color = vga::Color::LightGray;
const BG: vga::Color = vga::Color::White;

pub fn create(height: usize, width: usize) {
    for x in (3..height + 1) {
        for y in (2..width + 1) {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, SH, b' ');
        }
    }
    
    for x in (2..height) {
        for y in (1..width) {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, BG, b' ');
        }
    }
}

pub fn create_center() {
    let x1 = 25 / 4;
    let x2 = 25 - (25 / 4);
    
    let y1 = 79 / 4;
    let y2 = 79 - (79 / 4);

    for x in (x1 + 1..x2 + 1) {
        for y in (y1 + 1..y2 + 1) {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, SH, b' ');
        }
    }
    
    for x in (x1..x2) {
        for y in (y1..y2) {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, BG, b' ');
        }
    }

    for x in (x1 + 1..x2 - 1) {
        vga::WRITER.lock().write_xy_color_byte(x, y1, FG, BG, b'|');
        vga::WRITER.lock().write_xy_color_byte(x, y2 - 1, FG, BG, b'|');
    }

    for y in (y1 + 1..y2 - 1) {
        vga::WRITER.lock().write_xy_color_byte(x1, y, FG, BG, b'=');
        vga::WRITER.lock().write_xy_color_byte(x2 - 1, y, FG, BG, b'=');
    }
}