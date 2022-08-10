use core::fmt;
use std::{ops::Deref, rc::{Rc, Weak}, cell::RefCell, borrow::Borrow};

#[derive(Debug)]
enum ListNode<T> {
    Cons(T, List<T>),
    Nil,
}

struct List<T> {
    ptr: Rc<ListNode<T>>,
}

impl<T> List<T> {
    fn from_node(node: ListNode<T>) -> List<T> {
        List { ptr: Rc::new(node) }
    }

    fn clone(other: &List<T>) -> List<T> {
        List { ptr: Rc::clone(&other.ptr) }
    }

    pub fn new() -> List<T> {
        Self::from_node(ListNode::Nil)
    }

    pub fn tail(&self) -> Option<&List<T>> {
        match &*self.ptr {
            ListNode::Cons(_, rest) => Some(rest),
            ListNode::Nil => None,
        }
    }

    pub fn prepend(&self, first: T) -> List<T> {
        Self::from_node(ListNode::Cons(first, Self::clone(self)))
    }

    pub fn size(&self) -> u32 {
        match &*self.ptr {
            ListNode::Cons(_, rest) => rest.size() + 1,
            ListNode::Nil => 0,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.ptr {
            ListNode::Cons(first, rest) => write!(f, "({:?}, {:?})", first, rest),
            ListNode::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: RefCell<Vec<Rc<Node<T>>>>,
    parent: RefCell<Weak<Node<T>>>,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::new()
        .prepend(4)
        .prepend(3)
        .prepend(2)
        .prepend(1)
        .prepend(0);

    println!("list = {:?}", list);
    println!("list.length = {}", list.size());

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let me = MyBox::new(String::from("inhibitor"));
    hello(&me);

    let c = CustomSmartPointer {
        data: String::from("foo"),
    };

    let _d = CustomSmartPointer {
        data: String::from("bar"),
    };

    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main()");

    let a = List::new()
        .prepend(10)
        .prepend(5);

    println!("count after creating a = {:?} is {}", a, Rc::strong_count(&a.ptr));

    let b = a.prepend(3);

    println!("count after creating b = {:?} is {}", b, Rc::strong_count(&a.ptr));

    {
        let c = a.prepend(4);
        println!("count after creating c = {:?} is {}", c, Rc::strong_count(&a.ptr));
    }

    println!("count after c goes out of scope is {}", Rc::strong_count(&a.ptr));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
