// Draw Line: A monochrome screen is stored as a single array of bytes, allowing eight consecutive
// pixels to be stored in one byte. The screen has width w, where w is divisible by 8 (that is, no byte will
// be split across rows). The height of the screen, of course, can be derived from the length of the array
// and the width. Implement a function that draws a horizontal line from ( xl, y) to ( x2, y).
// The method signature should look something like:
// drawline(byte[] screen, int width, int x1, int x2, int y)

const FULL_MASK: u8 = 0b11111111;

fn draw_line(screen: &mut [u8], _width: u8, x1: u8, x2: u8, y: u8) {
    let start = (((y * 8) - 1) + ((x1 / 8) + 1), x1 % 8);
    let end = (((y * 8) - 1) + ((x2 / 8) + 1), 8 - x2 % 8);

    if start.0 != end.0 {
        for i in start.0..end.0 + 1 {
            let bit_mask = gen_bit_mask(i, start, end);
            let idx = i as usize;
            screen[idx] |= bit_mask;
        }
    } else {
        let mut mask = 0;
        for i in 0..8 {
            if i > start.1 && i <= 8 - end.1 {
                mask |= (1 << i);
            }
        }

        screen[start.0 as usize] |= mask;
    }
}

fn gen_bit_mask(pos: u8, start: (u8, u8), end: (u8, u8)) -> u8 {
    if pos > start.0 && pos < end.0 {
        FULL_MASK
    } else if pos == start.0 {
        FULL_MASK >> start.1
    } else {
        FULL_MASK << end.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_draw_line() {
        let mut screen = [0; 32];
        draw_line(&mut screen, 8, 27, 53, 2);
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0b00011111, 0b11111111,
                0b11111111, 0b11111000, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            screen
        );
    }

    #[test]
    fn should_draw_line_1() {
        let mut screen = [0; 16];
        draw_line(&mut screen, 8, 10, 13, 1);
        assert_eq!(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0b00111000, 0, 0, 0, 0, 0, 0],
            screen
        );
    }
}
