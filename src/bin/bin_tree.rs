use std::fmt::Debug;

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

    println!("{t:?}");

    t.print_lfirst(0);
}

#[derive(Debug)]
struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    fn new() -> Self {
        BinTree(None)
    }
}

impl<T: PartialOrd> BinTree<T> {
    fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))
            }
        }
    }
}

impl<T: Debug> BinTree<T> {
    fn print_lfirst(&self, dp: i32) {
        if let Some(ref db) = self.0 {
            db.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{spc}{:?}", db.data);
            db.right.print_lfirst(dp + 1);
        }
    }
}
