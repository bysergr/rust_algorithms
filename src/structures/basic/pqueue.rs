use std::cmp::PartialOrd;
use std::fmt::Debug;

/// Priority Queue is a `FIFO` Data Structure
///
/// `F` First
/// `I` In
/// `F` First
/// `O` Out
///
/// Return the items with more highests priority
/// first before elements with lower priority
///
#[derive(Debug)]
pub struct PQueue<T: PartialOrd + Debug> {
    items: Vec<T>,
    reverse: bool,
}

impl<T: PartialOrd + Debug> PQueue<T> {
    /// Return a new empty `Queue<T>`
    ///
    ///
    ///
    pub fn new(reverse: bool) -> Self {
        PQueue {
            items: Vec::new(),
            reverse,
        }
    }

    /// Return a boolean `true` if don't have Items
    pub fn is_empty(&mut self) -> bool {
        if self.items.len() > 0 {
            return false;
        }
        true
    }

    /// Return the number of Items
    pub fn size(&mut self) -> usize {
        self.items.len()
    }

    /// return the last Item
    pub fn front(&mut self) -> Option<&T> {
        return self.items.first();
    }

    /// return the last Item
    pub fn back(&mut self) -> Option<&T> {
        return self.items.last();
    }

    /// Add a new Item
    pub fn enqueue(&mut self, item: T) {
        if self.items.len() == 0 {
            self.items.push(item);
            return;
        }

        for index in 0..self.items.len() {
            if self.items[index] > item && !self.reverse {
                continue;
            }

            if self.items[index] < item && self.reverse {
                continue;
            }

            self.items.insert(index, item);
            return;
        }

        self.items.push(item)
    }

    /// Return and delete the first Item
    pub fn dequeue(&mut self) -> Option<T> {
        let len = self.size();
        if len == 0 {
            return None;
        }

        return Some(self.items.remove(0));
    }
}
