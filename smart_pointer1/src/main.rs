use std::ops::Deref;
use crate::List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let q = 5;
    let w = MyBox::new(q);

    assert_eq!(5, q);
    assert_eq!(5, *w);

    drop_test();
}

enum List {
    // Cons(i32, List), 컴파일러는 List가 얼마만큼의 공간을 할당할지 알 수 없어 컴파일을 거부한다.
    Cons(i32, Box<List>), // Cons는 Box를 통해 크기를 알 수 있다.
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 역참조를 하기 위해서는 deref 구현이 필요
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

fn drop_test() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // drop(c); 명시적으로 제거 가능
    // c.drop() <-- 컴파일러가 중복 해제 에러를 방지하기 위해 못하도록 막음
    println!("CustomSmartPointers created.");
}