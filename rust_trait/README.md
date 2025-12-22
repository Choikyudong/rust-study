# trait
1. 다른 언어에서는 인터페이스(interface) 라고 불린다.
2. 트레이트를 정의하는 방법은 다음과 같다.
   ```rust
   trait TraitName {
        // ..   
   }
   ```
   
   구현을 하기 위해서는 아래와 같이 한다.
   ```rust
   pub trait Animal {
       fn name(&self) -> String;
   }
   
   pub struct  Dog {
       pub name: String
   }
   
   impl Animal for Dog {
       fn name(&self) -> String {
           "Dog".to_string()
       }
   }
   ```