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
        self.word_count + if !self.is_whitespace { 1 } else { 0 }
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
    fn test_counters() {
        let cases: Vec<(Box<dyn Counter>, &str, usize)> = vec![
            (Box::new(ByteCounter::default()), "a💩", 5),
            (Box::new(CharacterCounter::default()), "a💩", 2),
            (Box::new(WordCounter::default()), " 💩 hello world", 3),
            (Box::new(WordCounter::default()), " 💩 hello \n world ", 3),
            (
                Box::new(NewlineCounter::default()),
                " 💩 hello \n world ",
                1,
            ),
            (Box::new(NewlineCounter::default()), " 💩 hello world \n", 1),
            (
                Box::new(NewlineCounter::default()),
                "\n💩 hell\no world \n",
                3,
            ),
            (Box::new(NewlineCounter::default()), "💩", 0),
            (
                Box::new(MaxLineLengthCounter::default()),
                " 💩 hello \n world ",
                9,
            ),
            (Box::new(MaxLineLengthCounter::default()), "\n\n \n", 1),
        ];

        for (mut counter, s, expected_count) in cases {
            for ch in s.chars() {
                counter.count(ch);
            }
            assert_eq!(counter.get(), expected_count);
            counter.reset();
            assert_eq!(counter.get(), 0);
        }
    }
}
