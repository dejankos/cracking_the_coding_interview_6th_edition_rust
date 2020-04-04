//Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is 4
//bytes, write a method to rotate the image by 90 degrees. Can you do this in place?

const N: usize = 3;

type Image = [[usize; N]; N];

fn rotate(img: &Image) -> Image {
    let mut r: Image = [[0; N]; N];
    for i in 0..img.len() {
        for j in 0..img.len() {
            r[i][j] = img[j][N - i - 1];
        }
    }

    r
}

fn _rotate_in_place(_img: &mut Image) -> &Image {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_rotate() {
        let mut img: Image = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected: Image = [[3, 6, 9], [2, 5, 8], [1, 4, 7]];

        assert_eq!(rotate(&mut img), expected);
    }
}
