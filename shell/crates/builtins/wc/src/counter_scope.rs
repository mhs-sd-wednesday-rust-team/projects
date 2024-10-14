use crate::counters::Counter;

#[derive(Default)]
pub struct CounterScope {
    counters: Vec<TotalCounter>,
}

impl CounterScope {
    pub fn add_counter<C: Counter + Default + 'static>(&mut self) {
        self.counters.push(TotalCounter::new_with_counter::<C>());
    }

    pub fn count(&mut self, ch: char) {
        for counter in self.counters.as_mut_slice() {
            counter.count(ch);
        }
    }

    pub fn reset(&mut self) -> Vec<usize> {
        self.counters
            .iter_mut()
            .map(|c| {
                let current = c.get();
                c.reset();
                current
            })
            .collect::<Vec<_>>()
    }

    pub fn total(&self) -> Vec<usize> {
        self.counters.iter().map(|c| c.total()).collect::<Vec<_>>()
    }

    pub fn is_empty(&self) -> bool {
        self.counters.is_empty()
    }
}

struct TotalCounter {
    counter: Box<dyn Counter>,
    total: usize,
}

impl Counter for TotalCounter {
    fn count(&mut self, ch: char) {
        self.counter.count(ch);
    }

    fn get(&self) -> usize {
        self.counter.get()
    }

    fn reset(&mut self) {
        self.total = self.aggregate(self.total, self.counter.get());
        self.counter.reset();
    }

    fn aggregate(&self, a: usize, b: usize) -> usize {
        self.counter.aggregate(a, b)
    }
}

impl TotalCounter {
    pub fn new_with_counter<C: Counter + Default + 'static>() -> Self {
        Self {
            counter: Box::new(C::default()),
            total: 0,
        }
    }

    pub fn total(&self) -> usize {
        self.aggregate(self.total, self.counter.get())
    }
}

#[cfg(test)]
mod tests {
    use crate::counters::{ByteCounter, CharacterCounter};

    use super::*;

    #[test]
    fn test_total() {
        let mut total_counter = TotalCounter::new_with_counter::<CharacterCounter>();

        total_counter.count('a');
        total_counter.count('🤯');

        assert_eq!(total_counter.get(), 2);
        assert_eq!(total_counter.total(), 2);

        total_counter.reset();

        assert_eq!(total_counter.get(), 0);
        assert_eq!(total_counter.total(), 2);

        total_counter.count('a');
        total_counter.count('🤯');

        assert_eq!(total_counter.get(), 2);
        assert_eq!(total_counter.total(), 4);

        total_counter.reset();

        assert_eq!(total_counter.get(), 0);
        assert_eq!(total_counter.total(), 4);
    }

    #[test]
    fn test_scope() {
        let mut scope = CounterScope::default();
        scope.add_counter::<ByteCounter>();
        scope.add_counter::<CharacterCounter>();

        scope.count('a');
        scope.count('🤯');

        assert_eq!(scope.total(), &[5, 2]);
        assert_eq!(scope.reset(), &[5, 2]);
        assert_eq!(scope.reset(), &[0, 0]);
        assert_eq!(scope.total(), &[5, 2]);

        scope.count('a');
        scope.count('🤯');

        assert_eq!(scope.total(), &[10, 4]);
        assert_eq!(scope.reset(), &[5, 2]);
        assert_eq!(scope.total(), &[10, 4]);
    }
}
