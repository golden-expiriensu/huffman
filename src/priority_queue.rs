#[derive(Debug, PartialEq)]
pub struct Elem<T> {
    pub weight: usize,
    pub value: T,
}

pub struct Queue<T>(Vec<Elem<T>>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn enqueue(&mut self, value: T, weight: usize) {
        match self.0.iter().position(|e| e.weight >= weight) {
            Some(index) => self.0.insert(index, Elem { value, weight }),
            None => self.0.push(Elem { value, weight }),
        }
    }

    pub fn dequeue(&mut self) -> Option<Elem<T>> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue(Vec::new());
        queue.enqueue('D', 10);
        queue.enqueue('B', 6);
        queue.enqueue('A', 11);
        queue.enqueue(' ', 10);
        queue.enqueue('C', 2);
        queue.enqueue('E', 7);

        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: 'A',
                weight: 11
            })
        );
        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: 'D',
                weight: 10
            })
        );
        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: ' ',
                weight: 10
            })
        );
        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: 'E',
                weight: 7
            })
        );
        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: 'B',
                weight: 6
            })
        );
        assert_eq!(
            queue.dequeue(),
            Some(Elem {
                value: 'C',
                weight: 2
            })
        );
    }
}
