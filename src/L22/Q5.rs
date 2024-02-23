// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello".to_string();
  let mut m2 = HashMap::new();
  // 所有权在这里发生了转移
  m2.insert(v2.clone(), v1);

  assert_eq!(v2, "hello");

   println!("Success!")
}


// Using &v2: You just borrow v2 and insert a reference to it in the HashMap. This way, 
// v2 isn't given away, and you can still use it later. The keys in the HashMap are 
// references to strings (&String), not the strings themselves. This is efficient because it doesn't copy v2.

// Using v2.clone(): You make a copy of v2 and insert that copy into the HashMap. 
// This means the HashMap gets its own version of v2, and you can still use the original v2 afterwards. 
// The keys in the HashMap are actual strings (String). This method uses more resources since it 
// duplicates v2, but it keeps the ownership rules of Rust happy by not sharing the original v2.