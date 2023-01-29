/// Stack is a `LIFO` Data Structure
///
/// `L` Last
/// `I` In
/// `F` First
/// `O` Out
///
/// Store objects and always return
/// the last object saved
///
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Return a new empty `Stack<T>`
    pub fn new() -> Self {
        Stack { items: Vec::new() }
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

    /// Return the last Item
    pub fn top(&mut self) -> Option<&T> {
        return self.items.last();
    }

    /// Add a new Item
    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }

    /// Return and delete the last Item
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
