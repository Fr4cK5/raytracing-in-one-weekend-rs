use crate::vec3::Color;

pub fn write_color(out: &mut Vec<String>, col: &Color) {
    let line = format!("{} {} {}",
        (col.0 * 255.99999) as u8,
        (col.1 * 255.99999) as u8,
        (col.2 * 256.99999) as u8,
    );
    out.push(line);
}
