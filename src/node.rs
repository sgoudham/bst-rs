use std::cmp::{max, Ordering};
use std::collections::VecDeque;

pub(crate) type HeapNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub(crate) struct Node<T: Ord> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
}

impl<T: Ord> Node<T> {
    pub(crate) fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub(crate) fn iterative_insert(mut root: &mut HeapNode<T>, value: T) -> Result<(), ()> {
        while let Some(ref mut node) = root {
            match value.cmp(&node.value) {
                Ordering::Equal => return Err(()),
                Ordering::Less => root = &mut node.left,
                Ordering::Greater => root = &mut node.right,
            }
        }
        *root = Some(Box::new(Node::new(value)));

        Ok(())
    }

    pub(crate) fn recursive_insert(&mut self, value: T) -> Result<(), ()> {
        match value.cmp(&self.value) {
            Ordering::Equal => Err(()),
            Ordering::Less => match self.left {
                None => {
                    self.left = Some(Box::from(Node::new(value)));
                    Ok(())
                }
                Some(ref mut node) => node.recursive_insert(value),
            },
            Ordering::Greater => match self.right {
                None => {
                    self.right = Some(Box::from(Node::new(value)));
                    Ok(())
                }
                Some(ref mut node) => node.recursive_insert(value),
            },
        }
    }

    pub(crate) fn iterative_contains(mut root: &HeapNode<T>, value: &T) -> bool {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return true,
                Ordering::Less => root = &current.left,
                Ordering::Greater => root = &current.right,
            }
        }

        false
    }

    pub(crate) fn recursive_contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match self.left {
                None => false,
                Some(ref node) => node.recursive_contains(value),
            },
            Ordering::Greater => match self.right {
                None => false,
                Some(ref node) => node.recursive_contains(value),
            },
        }
    }

    pub(crate) fn iterative_retrieve<'a>(mut root: &'a HeapNode<T>, value: &T) -> Option<&'a T> {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return Some(&current.value),
                Ordering::Less => root = &current.left,
                Ordering::Greater => root = &current.right,
            }
        }

        None
    }

    pub(crate) fn recursive_retrieve(&self, value: &T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref node) => node.recursive_retrieve(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref node) => node.recursive_retrieve(value),
            },
        }
    }

    pub(crate) fn iterative_retrieve_as_mut<'a>(
        mut root: &'a mut HeapNode<T>,
        value: &T,
    ) -> Option<&'a mut T> {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return Some(&mut current.value),
                Ordering::Less => root = &mut current.left,
                Ordering::Greater => root = &mut current.right,
            }
        }

        None
    }

    pub(crate) fn recursive_retrieve_as_mut(&mut self, value: &T) -> Option<&mut T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref mut node) => node.recursive_retrieve_as_mut(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref mut node) => node.recursive_retrieve_as_mut(value),
            },
        }
    }

    pub(crate) fn iterative_height(root: &HeapNode<T>) -> isize {
        let mut height = -1;
        let mut queue = VecDeque::new();
        queue.push_front(root);

        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                let current = queue.pop_front().as_ref().unwrap().as_ref().unwrap();
                if current.left.is_some() {
                    queue.push_back(&current.left);
                }
                if current.right.is_some() {
                    queue.push_back(&current.right);
                }
                size -= 1;
            }
            height += 1;
        }

        height
    }

    pub(crate) fn recursive_height(root: &HeapNode<T>) -> isize {
        match root {
            None => -1,
            Some(node) => {
                1 + max(
                    Node::recursive_height(&node.left),
                    Node::recursive_height(&node.right),
                )
            }
        }
    }

    pub(crate) fn iterative_remove(mut root: &mut HeapNode<T>, value: &T) -> Result<(), ()> {
        while let Some(ref mut current) = root {
            match value.cmp(&current.value) {
                Ordering::Less => root = &mut root.as_mut().unwrap().left,
                Ordering::Greater => root = &mut root.as_mut().unwrap().right,
                Ordering::Equal => {
                    match (current.left.as_mut(), current.right.as_mut()) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = current.left.take(),
                        (None, Some(_)) => *root = current.right.take(),
                        (Some(_), Some(_)) => {
                            root.as_mut().unwrap().value =
                                Node::iterative_remove_min(&mut current.right).unwrap()
                        }
                    }

                    return Ok(());
                }
            }
        }

        Err(())
    }

    pub(crate) fn recursive_remove(root: &mut HeapNode<T>, value: &T) -> Result<(), ()> {
        if let Some(ref mut node) = root {
            return match value.cmp(&node.value) {
                Ordering::Less => Node::recursive_remove(&mut node.left, value),
                Ordering::Greater => Node::recursive_remove(&mut node.right, value),
                Ordering::Equal => {
                    match (&node.left, &node.right) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = node.left.take(),
                        (None, Some(_)) => *root = node.right.take(),
                        (Some(_), Some(_)) => {
                            node.value = Node::recursive_remove_min(&mut node.right).unwrap()
                        }
                    }

                    Ok(())
                }
            };
        }

        Err(())
    }

    pub(crate) fn iterative_min(mut root: &HeapNode<T>) -> Option<&T> {
        while let Some(current) = root {
            if current.left.is_none() {
                return Some(&current.value);
            }
            root = &current.left;
        }

        None
    }

    pub(crate) fn recursive_min(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.value),
            Some(node) => node.recursive_min(),
        }
    }

    pub(crate) fn iterative_max(mut root: &HeapNode<T>) -> Option<&T> {
        while let Some(current) = root {
            if current.right.is_none() {
                return Some(&current.value);
            }
            root = &current.right;
        }

        None
    }

    pub(crate) fn recursive_max(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.value),
            Some(node) => node.recursive_max(),
        }
    }

    pub(crate) fn iterative_remove_min(mut root: &mut HeapNode<T>) -> Option<T> {
        if root.is_some() {
            while root.as_ref().unwrap().left.is_some() {
                root = &mut root.as_mut().unwrap().left
            }

            let node = root.take().unwrap();
            *root = node.right;
            return Some(node.value);
        }

        None
    }

    pub(crate) fn recursive_remove_min(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().left.is_some() {
            Node::recursive_remove_min(&mut root.as_mut().unwrap().left)
        } else {
            let node = root.take().unwrap();
            *root = node.right;
            Some(node.value)
        }
    }

    pub(crate) fn iterative_remove_max(mut root: &mut HeapNode<T>) -> Option<T> {
        if root.is_some() {
            while root.as_ref().unwrap().right.is_some() {
                root = &mut root.as_mut().unwrap().right
            }

            let node = root.take().unwrap();
            *root = node.left;
            return Some(node.value);
        }

        None
    }

    pub(crate) fn recursive_remove_max(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().right.is_some() {
            Node::recursive_remove_max(&mut root.as_mut().unwrap().right)
        } else {
            let node = root.take().unwrap();
            *root = node.left;
            Some(node.value)
        }
    }

    pub(crate) fn iterative_pre_order_vec(node: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack = vec![node.as_ref()];

        while let Some(current) = stack.pop().unwrap_or(None) {
            elements.push(&current.value);
            if current.right.is_some() {
                stack.push(current.right.as_ref());
            }
            if current.left.is_some() {
                stack.push(current.left.as_ref());
            }
        }

        elements
    }

    pub(crate) fn recursive_pre_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Node::recursive_pre_order_vec(&node.left, elements);
            Node::recursive_pre_order_vec(&node.right, elements);
        }
    }

    pub(crate) fn iterative_in_order_vec(mut root: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack = Vec::new();

        while !stack.is_empty() || root.is_some() {
            if root.is_some() {
                stack.push(root);
                root = &root.as_ref().unwrap().left;
            } else {
                let node = stack.pop().unwrap();
                elements.push(&node.as_ref().unwrap().value);
                root = &node.as_ref().unwrap().right;
            }
        }

        elements
    }

    pub(crate) fn recursive_in_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_in_order_vec(&node.left, elements);
            elements.push(&node.value);
            Node::recursive_in_order_vec(&node.right, elements);
        }
    }

    pub(crate) fn iterative_post_order_vec(root: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack_one = vec![root];
        let mut stack_two = vec![];

        while let Some(node) = stack_one.pop().unwrap_or(&None) {
            if node.left.is_some() {
                stack_one.push(&node.left);
            }
            if node.right.is_some() {
                stack_one.push(&node.right);
            }
            stack_two.push(node);
        }

        while let Some(node) = stack_two.pop() {
            elements.push(&node.value);
        }

        elements
    }

    pub(crate) fn recursive_post_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_post_order_vec(&node.left, elements);
            Node::recursive_post_order_vec(&node.right, elements);
            elements.push(&node.value);
        }
    }

    pub(crate) fn iterative_level_order_vec(root: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_front(root.as_ref());

        while let Some(current) = deque.pop_front().unwrap_or(None) {
            elements.push(&current.value);
            if current.left.is_some() {
                deque.push_back(current.left.as_ref());
            }
            if current.right.is_some() {
                deque.push_back(current.right.as_ref());
            }
        }

        elements
    }

    pub(crate) fn recursive_level_order_vec<'a>(root: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        let height = Node::recursive_height(root);
        for i in 1..=height + 1 {
            Node::recursive_current_level(root, elements, i);
        }
    }

    fn recursive_current_level<'a>(root: &'a HeapNode<T>, elements: &mut Vec<&'a T>, level: isize) {
        if root.is_some() {
            match level.cmp(&1) {
                Ordering::Less => {}
                Ordering::Equal => elements.push(&root.as_ref().unwrap().value),
                Ordering::Greater => {
                    Node::recursive_current_level(
                        &root.as_ref().unwrap().left,
                        elements,
                        level - 1,
                    );
                    Node::recursive_current_level(
                        &root.as_ref().unwrap().right,
                        elements,
                        level - 1,
                    );
                }
            }
        }
    }

    pub(crate) fn iterative_consume_pre_order_vec(node: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack = vec![node];

        while let Some(current) = stack.pop().unwrap_or(None) {
            elements.push(current.value);
            if current.right.is_some() {
                stack.push(current.right);
            }
            if current.left.is_some() {
                stack.push(current.left);
            }
        }

        elements
    }

    pub(crate) fn recursive_consume_pre_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Node::recursive_consume_pre_order_vec(node.left, elements);
            Node::recursive_consume_pre_order_vec(node.right, elements);
        }
    }

    pub(crate) fn iterative_consume_in_order_vec(root: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack = vec![root];

        while !stack.is_empty() {
            if let Some(mut current) = stack.pop().unwrap() {
                if current.left.is_some() {
                    let left_node = current.left.take();
                    stack.push(Some(current));
                    stack.push(left_node);
                } else {
                    let right_node = current.right.take();
                    elements.push(current.value);
                    stack.push(right_node);
                }
            }
        }

        elements
    }

    pub(crate) fn recursive_consume_in_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_in_order_vec(node.left, elements);
            elements.push(node.value);
            Node::recursive_consume_in_order_vec(node.right, elements);
        }
    }

    pub(crate) fn iterative_consume_post_order_vec(root: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack_one = vec![root];
        let mut stack_two = vec![];

        while let Some(mut node) = stack_one.pop().unwrap_or(None) {
            if let Some(left_node) = node.left.take() {
                stack_one.push(Some(left_node));
            }
            if let Some(right_node) = node.right.take() {
                stack_one.push(Some(right_node));
            }
            stack_two.push(node);
        }

        while let Some(node) = stack_two.pop() {
            elements.push(node.value);
        }

        elements
    }

    pub(crate) fn recursive_consume_post_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_post_order_vec(node.left, elements);
            Node::recursive_consume_post_order_vec(node.right, elements);
            elements.push(node.value);
        }
    }

    pub(crate) fn iterative_consume_level_order_vec(root: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_front(root);

        while let Some(current) = deque.pop_front().unwrap_or(None) {
            elements.push(current.value);
            if current.left.is_some() {
                deque.push_back(current.left);
            }
            if current.right.is_some() {
                deque.push_back(current.right);
            }
        }

        elements
    }

    pub(crate) fn recursive_consume_level_order_vec(root: HeapNode<T>, elements: &mut Vec<T>) {
        let height = Node::recursive_height(&root);
        for i in 0..height + 1 {
            // SAFETY: this is sound because dealloc_boxes ensures that the elements don't
            // get dropped again
            unsafe { Node::write_level_into_vec(&root, elements, i) };
        }
        Node::dealloc_boxes(root);
    }

    /// # Safety
    ///
    /// The caller must ensure that the values contained in the heap are not dropped again.
    ///
    /// Otherwise this could lead to a double free.
    unsafe fn write_level_into_vec(root: &HeapNode<T>, elements: &mut Vec<T>, level: isize) {
        if let Some(node) = root {
            if level == 0 {
                // "move" the value without actually moving
                let element = std::ptr::read(&node.value);
                elements.push(element);
            } else {
                Node::write_level_into_vec(&node.left, elements, level - 1);
                Node::write_level_into_vec(&node.right, elements, level - 1);
            }
        }
    }

    fn dealloc_boxes(root: HeapNode<T>) {
        if let Some(node) = root {
            // move out of the box by de-referencing to drop it and destructure the `Node`
            let Node { value, left, right } = *node;
            // ensure that the value is not dropped again by forgetting it
            std::mem::forget(value);
            Node::dealloc_boxes(left);
            Node::dealloc_boxes(right);
        }
    }
}
