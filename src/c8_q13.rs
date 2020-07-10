// You have a stack of n boxes, with widths w 1 , heights h i , and depths d i . The boxes
// cannot be rotated and can only be stacked on top of one another if each box in the stack is strictly
// larger than the box above it in width, height, and depth. Implement a method to compute the
// height of the tallest possible stack. The height of a stack is the sum of the heights of each box.

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Box {
    w: usize,
    h: usize,
    d: usize,
}

fn sort_boxes(mut boxes: Vec<Box>) -> Vec<Box> {
    boxes.sort();
    boxes
}

fn find_tallest(boxes: Vec<Box>) -> usize {
    let sorted = sort_boxes(boxes);
    sorted
        .iter()
        .enumerate()
        .map(|(i, _b)| r_find_tallest(&sorted, i))
        .max()
        .unwrap_or(0)
}

fn r_find_tallest(boxes: &Vec<Box>, index: usize) -> usize {
    let current = boxes.get(index).unwrap();
    let tallest = boxes
        .iter()
        .enumerate()
        .filter(|(i, b)| *i > index && can_place_box(b, current))
        .map(|(i, _)| r_find_tallest(boxes, i))
        .max()
        .unwrap_or(0);

    tallest + current.h
}

fn can_place_box(box_: &Box, prev: &Box) -> bool {
    box_.w > prev.w && box_.h > prev.h && box_.d > prev.d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_tallest_stack() {
        let boxes = vec![
            Box { w: 1, h: 1, d: 1 },
            Box { w: 4, h: 4, d: 4 },
            Box { w: 3, h: 3, d: 3 },
            Box { w: 2, h: 2, d: 2 },
            Box { w: 1, h: 3, d: 2 },
        ];

        assert_eq!(10, find_tallest(boxes));
    }
}
