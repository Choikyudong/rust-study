# 구조체
1. 구조체란 여러개의 값을 묶어 하나의 데이터로 정의할 수 있다.
   비슷한 형태로는 c의 구조체, 객체지향이 있다.
2. 구조체 정의
   struct <구조체 이름> {
      필드명: <속성>,
      필드명: <속성>,
   }
3. 구조체를 생성하기 위해서는 인스턴스를 생성해야 한다.
   필드의 순서는 구조체 정의와 동일하지 않아도 된다.
4. [인스턴스.필드명] 으로 데이터를 다룰 수 있다.
5. 변수명과 구조체 필드명이 같을 때는 필드 초기화 축약법이 사용 가능하다.
6. 구조체 업데이트 문법 사용시 업데이트할 필드를 먼저 작성하고 복사할 인스턴스를 마지막에 적는다.
7. 명명된 플드가 없는 튜플 구조체
   튜플과 유사한 형태를 지닌 튜플 구조체가 있다.
   struct <구조체 이름>(속성..);
   속성은 서로다르게 정의하여도 된다.
8. 필드가 없는 구조체도 정의가 가능하다.
   어떤 타입에 대해 트레이트를 구현하고 싶을 때 사용된다.