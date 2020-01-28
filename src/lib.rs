//! Amortized constant-time queue of references using
//! "lazy queuing".

/// Queue of references of type `T` with minimum lifetime
/// `'a`.
pub struct AmoQueue<'a, T> {
    head: Vec<&'a T>,
    tail: Vec<&'a T>,
}

impl<'a, T> AmoQueue<'a, T> {

    /// Create a new `AmoQueue`.
    pub fn new() -> Self {
        let head = Vec::new();
        let tail = Vec::new();
        Self { head, tail }
    }

    /// Number of elements currently contained in the queue.
    pub fn len(&self) -> usize {
        self.head.len() + self.tail.len()
    }

    /// True iff the queue currently contains 0 elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a reference onto the tail of the queue.
    pub fn push_tail(&mut self, elem: &'a T) {
        self.tail.push(elem);
    }

    /// Pop a reference off the head of the queue.
    pub fn pop_head(&mut self) -> Option<&'a T> {
        if self.is_empty() {
            return None;
        }
        if self.head.is_empty() {
            self.head.extend(self.tail.drain(..).rev());
        }
        self.head.pop()
    }
}

#[test]
fn test_basic() {
    let s = [1, 2];
    let mut q = AmoQueue::new();
    q.push_tail(&s[0]);
    q.push_tail(&s[1]);
    assert_eq!(Some(&1), q.pop_head());
    q.push_tail(&3);
    assert_eq!(Some(&2), q.pop_head());
    assert_eq!(Some(&3), q.pop_head());
    assert!(q.is_empty());
    assert_eq!(None, q.pop_head());
}
