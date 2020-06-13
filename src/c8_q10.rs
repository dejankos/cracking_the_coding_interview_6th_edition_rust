// Paint Fill: Implement the "paint fill" function that one might see on many image editing programs.
// That is, given a screen (represented by a two-dimensional array of colors), a point, and a new color,
// fill in the surrounding area until the color changes from the original color

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Color {
    R,
    G,
    B,
}

fn fill(img: &mut Vec<Vec<Color>>, x: usize, y: usize, c: Color) {
    fill_img(img, x, y, img[x][y], c)
}

fn fill_img(img: &mut Vec<Vec<Color>>, x: usize, y: usize, from: Color, to: Color) {
    if x < img.len() && y < img[0].len() && img[x][y] == from {
        img[x][y] = to;

        fill_img(img, x + 1, y, from, to);
        if x >= 1 {
            fill_img(img, x - 1, y, from, to);
        }
        fill_img(img, x, y + 1, from, to);
        if y >= 1 {
            fill_img(img, x, y - 1, from, to);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fill_img() {
        let mut img = vec![
            vec![Color::B, Color::G, Color::B],
            vec![Color::B, Color::G, Color::G],
            vec![Color::B, Color::G, Color::B],
        ];

        fill(&mut img, 1, 1, Color::R);

        assert_eq!(
            vec![
                vec![Color::B, Color::R, Color::B],
                vec![Color::B, Color::R, Color::R],
                vec![Color::B, Color::R, Color::B]
            ],
            img
        );
    }
}
