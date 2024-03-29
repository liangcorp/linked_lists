#[derive(Debug)]
#[allow(dead_code)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            next: None,
            element: data,
        }
    }

    fn add(&mut self, data: T) {
        let element = self.element;
        let next = self.next.take();

        let previous_pointer = Box::new(Node { element, next });
        self.element = data;
        self.next = Some(previous_pointer);
    }

    fn display_list(&self) {
        let mut next_node = &self.next;

        println!("{:?}", self.element);

        while let Some(node) = &next_node {
            println!("{:?}", node.element);
            next_node = &node.next;
        }
    }
}

fn main() {
    let mut single_linked_list = Node::new(3);
    single_linked_list.add(4);
    single_linked_list.display_list();
    single_linked_list.add(5);
    single_linked_list.display_list();
}
