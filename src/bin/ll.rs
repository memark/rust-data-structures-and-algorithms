use std::fmt::{Debug, Display};

fn main() {
    let mut ll = LinkedList::new();

    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);
    ll.push_back(24);
    println!("{ll:?}");

    ll.push_sorted(5);
    println!("{ll:?}");

    println!("{:#?}", ll.all_values());
    println!("{:?}", ll.all_values());
    println!("{}", ll);
}

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Clone + Debug> LinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn push_sorted(&mut self, data: T) {
        dbg!(&data);
        dbg!(&self.0);
        match &mut self.0 {
            Some((d, ref mut child)) if data > *d => child.push_sorted(data),
            Some(_) => self.push_front(data),
            None => self.push_front(data),
        }
    }

    pub fn all_values(&self) -> Vec<T> {
        match &self.0 {
            Some((d, child)) => {
                let mut temp = [d.clone()].to_vec();
                temp.append(&mut child.all_values());
                temp
            }
            None => [].to_vec(),
        }
    }
}

impl<T: PartialOrd + Clone + Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.all_values()
                .iter()
                .map(|v| format!("{v:?}"))
                .collect::<Vec<_>>()
                .join(" -> ")
        )
    }
}
