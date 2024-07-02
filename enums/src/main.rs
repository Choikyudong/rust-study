// 열거형 선언 방법
enum IpAddrKind {
    V4,
    V6
}

//구조체에도 적용이 가능하다.
struct IpAddrStruct {
    kind: IpAddrKind, 
    address: String,
}


// 열거형의 배리언트에 데이터를 직접 넣을 수 있다.
enum IpAddr {
    V4(String),
    V6(String),
}

// 각 배리언트에 서로 다른 데이터 타입을 지정할 수 있다.
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 각 배리언트에 다른 종류의 타입들을 지정할 수 있다.
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 구조체같은 형태
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 위의 열거형을 구조체로 만든다면 아래와 같이 만들 수 있다.
struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

// 열거형의 메서드
impl Message {
    fn call(&self) {
        // 본문
    }
}

fn main() {
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // String 인수를 받아 IpAddr 인스턴스를 만드는것과 같다.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let soume_number = Some(5);
    let none: Option<i8> = None; // None == null
}
