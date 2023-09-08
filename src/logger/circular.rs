use std::iter;

pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    next_write_pos: usize,
}

impl<T> CircularBuffer<T> {
    /// Create a new CircularBuffer, which can hold max_depth elements
    pub fn new(max_depth: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: Vec::with_capacity(max_depth),
            next_write_pos: 0,
        }
    }
    /// Return the number of elements present in the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    /// Push a new element into the buffer.
    /// Until the capacity is reached, elements are pushed.
    /// Afterwards the oldest elements will be overwritten.
    pub fn push(&mut self, elem: T) {
        let max_depth = self.buffer.capacity();
        if self.buffer.len() < max_depth {
            self.buffer.push(elem);
        } else {
            self.buffer[self.next_write_pos % max_depth] = elem;
        }
        self.next_write_pos += 1;
    }
    /// Take out all elements from the buffer, leaving an empty buffer behind
    pub fn take(&mut self) -> Vec<T> {
        let mut consumed = vec![];
        let max_depth = self.buffer.capacity();
        if self.buffer.len() < max_depth {
            consumed.append(&mut self.buffer);
        } else {
            let pos = self.next_write_pos % max_depth;
            let mut xvec = self.buffer.split_off(pos);
            consumed.append(&mut xvec);
            consumed.append(&mut self.buffer)
        }
        self.next_write_pos = 0;
        consumed
    }
    /// Total number of elements pushed into the buffer.
    pub fn total_elements(&self) -> usize {
        self.next_write_pos
    }
    /// Return an iterator to step through all elements in the sequence,
    /// as these have been pushed (FIFO)
    pub fn iter(&mut self) -> iter::Chain<std::slice::Iter<T>, std::slice::Iter<T>> {
        let max_depth = self.buffer.capacity();
        if self.next_write_pos <= max_depth {
            // If buffer is not completely filled, then just iterate through it
            self.buffer.iter().chain(self.buffer[..0].iter())
        } else {
            let wrap = self.next_write_pos % max_depth;
            let it_end = self.buffer[..wrap].iter();
            let it_start = self.buffer[wrap..].iter();
            it_start.chain(it_end)
        }
    }
    /// Return an iterator to step through all elements in the reverse sequence,
    /// as these have been pushed (LIFO)
    pub fn rev_iter(
        &mut self,
    ) -> iter::Chain<std::iter::Rev<std::slice::Iter<T>>, std::iter::Rev<std::slice::Iter<T>>> {
        let max_depth = self.buffer.capacity();
        if self.next_write_pos <= max_depth {
            // If buffer is not completely filled, then just iterate through it
            self.buffer
                .iter()
                .rev()
                .chain(self.buffer[..0].iter().rev())
        } else {
            let wrap = self.next_write_pos % max_depth;
            let it_end = self.buffer[..wrap].iter().rev();
            let it_start = self.buffer[wrap..].iter().rev();
            it_end.chain(it_start)
        }
    }
}
