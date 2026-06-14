#[derive(Debug, Clone)]
pub struct RollingBuffer {
    buffer: String,
    max_len: usize,
}

impl RollingBuffer {
    pub fn new(max_len: usize) -> Self {
        Self {
            buffer: String::new(),
            max_len,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
        if self.buffer.len() > self.max_len {
            self.buffer.drain(..self.buffer.len() - self.max_len);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn content(&self) -> &str {
        &self.buffer
    }

    pub fn ends_with(&self, s: &str) -> bool {
        self.buffer.ends_with(s)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_content() {
        let mut buf = RollingBuffer::new(10);
        buf.push('h');
        buf.push('e');
        buf.push('l');
        assert_eq!(buf.content(), "hel");
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_max_len() {
        let mut buf = RollingBuffer::new(5);
        for c in "hello world".chars() {
            buf.push(c);
        }
        assert_eq!(buf.content(), "world");
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn test_ends_with() {
        let mut buf = RollingBuffer::new(20);
        for c in "type :greet".chars() {
            buf.push(c);
        }
        assert!(buf.ends_with(":greet"));
        assert!(!buf.ends_with(":nope"));
    }

    #[test]
    fn test_clear() {
        let mut buf = RollingBuffer::new(10);
        buf.push('a');
        buf.push('b');
        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }
}
