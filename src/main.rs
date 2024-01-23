fn main1() {
   let x = 5;
   // 填写空白处
   let p = &x;

   println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

fn main2() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}

fn main3() {
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}

// fn main4() {
//     let mut s = String::from("hello, ");

//     push_str4(&mut s)
// }

// fn push_str4(s: mut String) {
//     s.push_str("world")
// }

fn main() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let p = &mut s;
    
    p.push_str("world");
}