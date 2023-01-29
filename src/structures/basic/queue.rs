/// Queue is a `FIFO` Data Structure
///
/// `F` First
/// `I` In
/// `F` First
/// `O` Out
///
/// Store objects and always return
/// the first object saved
///
#[derive(Debug)]
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// Return a new empty `Queue<T>`
    pub fn new() -> Self {
        Queue { items: Vec::new() }
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
