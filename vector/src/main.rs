fn main() {
    // 백터 생성
    let _v: Vec<i32> = Vec::new();

    // 백터 선언과 초기화
    let mut v: Vec<i32> = vec![1, 2, 3];

    // 백터 사용
    v.push(4);
    println!("{:?}", v.get(0)); // Some(1) 로 출력됨
    println!("{:?}", &v[0]);

    /*
    러스트의 벡터는 값 추가시 새로운 공간으로 가져갈 가능성이 있고
    그럴 경우 first는 빈 공간을 가리켜 null이 될 수 있어 컴파일러가 아래 코드를 방지함

    let first = &v[0];
    v.push(6);
    println!("{:?}", first);
    */

    {
        let _v = vec![1, 2, 3];
    } // 이 시점에서 v는 사라지며 요소또한 사라진다.

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50; // 역참조 기능으로 포인터와 유사
    }
    println!("{:?}", v);
}
