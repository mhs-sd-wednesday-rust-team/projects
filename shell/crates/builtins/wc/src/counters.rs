pub trait Counter {
    fn count(&mut self, ch: char);
    fn get(&self) -> usize;
    fn reset(&mut self);
    fn aggregate(&self, a: usize, b: usize) -> usize;
}

#[derive(Default)]
pub struct ByteCounter(usize);

impl Counter for ByteCounter {
    fn count(&mut self, ch: char) {
        self.0 = self.aggregate(self.0, ch.len_utf8());
    }

    fn get(&self) -> usize {
        self.0
    }

    fn reset(&mut self) {
        *self = Default::default();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        a + b
    }
}

#[derive(Default)]
pub struct CharacterCounter(usize);

impl Counter for CharacterCounter {
    fn count(&mut self, _: char) {
        self.0 = self.aggregate(self.0, 1);
    }

    fn get(&self) -> usize {
        self.0
    }

    fn reset(&mut self) {
        *self = Default::default();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        a + b
    }
}

pub struct WordCounter {
    is_whitespace: bool,
    word_count: usize,
}

impl Default for WordCounter {
    fn default() -> Self {
        Self {
            is_whitespace: true,
            word_count: 0,
        }
    }
}

impl Counter for WordCounter {
    fn count(&mut self, ch: char) {
        if ch.is_whitespace() && !self.is_whitespace {
            self.is_whitespace = true;
            self.word_count = self.aggregate(self.word_count, 1);
        } else if !ch.is_whitespace() && self.is_whitespace {
            self.is_whitespace = false;
        }
    }

    fn get(&self) -> usize {
        self.word_count
    }

    fn reset(&mut self) {
        *self = Default::default();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        a + b
    }
}

#[derive(Default)]
pub struct NewlineCounter(usize);

impl Counter for NewlineCounter {
    fn count(&mut self, ch: char) {
        if ch == '\n' {
            self.0 = self.aggregate(self.0, 1);
        }
    }

    fn get(&self) -> usize {
        self.0
    }

    fn reset(&mut self) {
        *self = Default::default();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        a + b
    }
}

#[derive(Default)]
pub struct MaxLineLengthCounter {
    curr_length: usize,
    best_length: usize,
}

impl Counter for MaxLineLengthCounter {
    fn count(&mut self, ch: char) {
        if ch == '\n' {
            self.best_length = self.aggregate(self.best_length, self.curr_length);
            self.curr_length = 0;
        } else {
            self.curr_length += 1;
        }
    }

    fn get(&self) -> usize {
        self.best_length
    }

    fn reset(&mut self) {
        *self = Default::default();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        usize::max(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_counter() {
        let mut counter = ByteCounter::default();

        counter.count('a');
        counter.count('💩');

        assert_eq!(counter.get(), 5);

        counter.reset();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_char_counter() {
        let mut counter = CharacterCounter::default();

        counter.count('a');
        counter.count('💩');

        assert_eq!(counter.get(), 2);

        counter.reset();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_word_counter() {
        let mut counter = WordCounter::default();

        counter.count('a');
        counter.count(' ');
        counter.count('b');
        counter.count('\t');
        counter.count('c');
        counter.count('\n');
        counter.count('\r');
        counter.count('d');
        counter.count(' ');

        assert_eq!(counter.get(), 4);

        counter.reset();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_newline_counter() {
        let mut counter = NewlineCounter::default();

        counter.count('a');
        counter.count(' ');
        counter.count('b');
        counter.count('\t');
        counter.count('c');
        counter.count('\n');
        counter.count('\r');
        counter.count('d');
        counter.count(' ');
        counter.count('\n');

        assert_eq!(counter.get(), 2);

        counter.reset();

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_newline_counter_no_newline() {
        let mut counter = NewlineCounter::default();

        counter.count('a');
        counter.count(' ');
        counter.count('b');

        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_maxline_counter() {
        let mut counter = MaxLineLengthCounter::default();

        counter.count('a');
        counter.count(' ');
        counter.count('b');
        counter.count('\t');
        counter.count('c');
        counter.count('\n');
        counter.count('d');
        counter.count(' ');
        counter.count('\n');

        assert_eq!(counter.get(), 5);

        counter.reset();

        assert_eq!(counter.get(), 0);
    }
}
