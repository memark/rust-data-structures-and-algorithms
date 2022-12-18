use std::{cmp::max, fmt::Debug};

fn main() {
    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);

    t.print_lfirst();
    // println!("----------");
    // t.rot_left();
    // t.print_lfirst(0);
    // println!("----------");
    // t.rot_right();
    // t.print_lfirst(0);

    for i in 0..100_000 {
        t.add_sorted(i);
    }
    t.print_lfirst();
}

#[derive(Debug)]
struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
struct BinData<T> {
    data: T,
    h: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    fn rot_left(mut self) -> Box<Self> {
        // Result is the right node.
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), // Can't rotate without a right node.
        };

        // Move left of right node to right of start node.
        self.right = BinTree(res.left.0.take());
        self.right.set_height();

        // Set the result's left node to the start node.
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + max(res.left.height(), res.right.height());

        res
    }

    fn rot_right(mut self) -> Box<Self> {
        // Result is the left node.
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), // Can't rotate without a left node.
        };

        // Move right of left node to left of start node.
        self.left = BinTree(res.right.0.take());
        self.left.set_height();

        // Set the result's right node to the start node.
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + max(res.left.height(), res.right.height());

        res
    }
}

impl<T> BinTree<T> {
    fn new() -> Self {
        BinTree(None)
    }

    fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + max(t.left.height(), t.right.height());
        }
    }

    fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T: PartialOrd> BinTree<T> {
    fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    #[allow(clippy::bool_to_int_with_if)]
                    if bd.left.height() - bd.right.height() >= 2 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() >= 2 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };

        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            0 => self.set_height(),
            _ => panic!(),
        }
    }
}

impl<T: Debug> BinTree<T> {
    fn print_lfirst(&self) {
        self.print_lfirst_with_depth(0);
    }

    fn print_lfirst_with_depth(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst_with_depth(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst_with_depth(dp + 1);
        }
    }
}
