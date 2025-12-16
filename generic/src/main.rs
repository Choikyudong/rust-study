struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<X1, Y1> {
    x1: X1,
    y1: Y1
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mix_up<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x1: self.x1,
            y1: other.y1
        }
    }
}

fn main() {
    let point = Point{x: 1, y: 2};
    println!("{:?}", point.x());

    let point2 = Point2{x1: "Hello", y1: 'C'};
    let point3 = Point2{x1: 10, y1: '😂'};
    println!("x1: {}, x2: {}", &point2.x1, &point2.y1);

    let point3 = point3.mix_up(point2);
    println!("x1: {}, x2: {}", point3.x1, point3.y1);
}


