// Tower of Hanoi is a mathematical puzzle where we have three rods (1, 2, and 3) and N disks.
// Initially, all the disks are stacked in decreasing value of diameter i.e., the smallest disk
// is placed on the top. The objective of the puzzle is to move the entire stack to another rod,
// obeying the following simple rules:

// 1: Only one disk can be moved at a time.
// 2: Each move consists of taking the upper disk from one of the stacks and placing it on top
//        of another stack i.e. a disk can only be moved if it is the uppermost disk on a stack.
// 3: No disk may be placed on top of a smaller disk.

fn hanoi(v: &mut Vec<(u8, u8)>, n: usize, start: u8, end: u8) {
    if n == 1 {
        v.push((start, end));
    } else {
        let other = 6 - (start + end);
        hanoi(v, n - 1, start, other);
        v.push((start, end));
        hanoi(v, n - 1, other, end);
    }
}

pub fn tower_of_hanoi(n: usize, start: u8, end: u8) -> Vec<(u8, u8)> {
    // n is the number of disks in the start stack
    // id's of towers are 1, 2, 3
    let mut v = Vec::with_capacity(usize::pow(2, n as u32));
    hanoi(&mut v, n, start, end);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            // 1 is the start tower
            // 3 is the end tower
            tower_of_hanoi(2, 1, 3),
            [(1, 2), (1, 3), (2, 3)]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            tower_of_hanoi(3, 1, 3),
            [(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)]
        );
    }
}
