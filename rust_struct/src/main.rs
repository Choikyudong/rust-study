struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Name_Card(char, String);
struct UnitLikeStruct; // 유사 유닛 구조체(unit-like structs)라고 지칭한다.

fn main() {
    let mut user1 = User { // 자바스크립트 객체 선언하듯 하면 된다.
        active: true,
        user_name: String::from("userName"),
        email: String::from("userMail@example.com"),
        sign_in_count: 1,
    };
    user1.user_name = String::from("userNameChange");

    let user2 = build_user("example1".to_string(), "example2".to_string());

    // 기존 인스턴스를 이용해 새 인스턴스를 만들 수 있다.
    let user3 = User {
        active: user1.active,
        user_name: user1.user_name,
        email: user1.email,
        sign_in_count: user1.sign_in_count
    };

    // 구조체 업데이트 문법을 통해 명시된 필드만 간단하게 업데이트 시킬 수 있다.
    let user4 = User {
        active: false,
        ..user3
    };

    let black = Color(0, 0, 0);
    let origin = Point(5, 0, 0);
    let user_card = Name_Card('A', "Kyu".to_string());

    let non_filed = UnitLikeStruct;
}

// 함수를 통해 구조체를 다룰 수 있다.
fn build_user(email: String, user_name: String) -> User {
    /* 보통적인 방법
    User {
        active: true,
        user_name: user_name,
        email: email,
        sign_in_count: 1,
    }
    */

    // 필드 초기화 축약법
    User {
      active: true,
      user_name,
      email,
      sign_in_count: 1,
  }
}
