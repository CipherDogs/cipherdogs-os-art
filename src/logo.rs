use crate::vga;

const FG: vga::Color = vga::Color::Black;
const BG: vga::Color = vga::Color::Black;

pub fn draw(x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 {
        vga::WRITER.lock().write_xy_color_byte(x, y1, FG, BG, b' ');
        vga::WRITER.lock().write_xy_color_byte(x, y2, FG, BG, b' ');
    }

    for y in y1..y2 + 1 {
        vga::WRITER.lock().write_xy_color_byte(x1, y, FG, BG, b' ');
        vga::WRITER.lock().write_xy_color_byte(x2, y, FG, BG, b' ');
    }

    let dogy1 = y1 + ((y2 - y1) / 3);
    let dogy2 = y2 - ((y2 - y1) / 3);

    let dogx1 = x2 - ((y2 - y1) / 4);
    let dogx2 = x1 + ((y2 - y1) / 4);

    for y in dogy1..dogy2 {
        for x in dogx2..dogx1 + 1 {
            vga::WRITER.lock().write_xy_color_byte(x, y, FG, BG, b' ');
        }
    }

    vga::WRITER
        .lock()
        .write_xy_color_byte(dogx2, dogy2, FG, BG, b' ');
    vga::WRITER
        .lock()
        .write_xy_color_byte(dogx2 + 1, dogy2, FG, BG, b' ');
    vga::WRITER
        .lock()
        .write_xy_color_byte(dogx2 + 1, dogy2 + 1, FG, BG, b' ');

    vga::WRITER
        .lock()
        .write_xy_color_byte(dogx2 - 1, dogy2 - 1, FG, BG, b' ');
    vga::WRITER
        .lock()
        .write_xy_color_byte(dogx2 - 1, dogy1, FG, BG, b' ');
}
