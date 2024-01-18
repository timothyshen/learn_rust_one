fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn main1() {
    let x = 1;
    let y = 2;
    let result = add_with_extra(x, y);
    assert_eq!(result, 8);
}

fn main2() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);
}



fn main() {
   let v = {
        let x = 3;
        x
   };

   assert!(v == 3);
}


fn mai() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}