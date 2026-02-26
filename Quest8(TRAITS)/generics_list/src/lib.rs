#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }
     pub fn push(&mut self, value: T) {
        let new_list = match self.head.take() {
            Some(node) => Some(Node {
                value: value,
                next: Some(Box::new(node)),
            }),
            None => Some(Node {
                value: value,
                next: None,
            }),
        };
        self.head = new_list;
    }
    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }

    pub fn len(&self) -> usize {
        let mut cmp: usize = 0;
        let mut current: Option<&Node<T>> = self.head.as_deref();
        while let Some(node) = current { // Repeat this block as long as current contains a real node (not nothing).
            cmp += 1;
            current = node.next.as_deref();
        }

        cmp
    }
}
