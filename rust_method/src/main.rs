struct Test {
    number1: u32,
    number2: u32,
}

impl Test {
    fn method(self: &Self) -> u32 {
        self.number1 * self.number2
    }
    fn method2(&self) -> bool { // self: &Self와 같음
        self.number1 > 1
    }
    fn method3(&self, other: &Test) -> bool { // 같은 구조체를 넣어 비교도 가능하다.
        self.number2 > other.number2
    }
}

impl Test {
    fn another_method1(&self) -> char {
        // 메서드를 여러개 생성도 가능하다.
        'A'
    }
}

fn main() {
    let test1 = Test {
        number1: 365,
        number2: 93,
    };

    println!("Test 구조체의 숫자의 곱은? {}", test1.method());
    println!("number1 은 1보다 큰가요? {}", test1.method2());

    let test2 = Test {
        number2: 313,
        ..test1
    };
    println!("test2의 number2가 더 큰가요? {}", test2.method3(&test1));
}
