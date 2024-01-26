// 修复错误，不要新增代码行
fn main1() {
    let s: &str = "hello, world";
}
// fn main2() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

fn main3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}
// 修复所有错误，并且不要新增代码行
fn main4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}

fn main5() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

fn main6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    format!("{}", &s1);
}


// // 使用至少两种方法来修复错误
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }

// fn greetings(s:String) {
//     println!("{}",s)
// }


// 使用至少两种方法来修复错误
fn main() {
    let s = "hello, world".to_string();
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}

// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = &s;
// }
// // 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world";
//     let s1: &str = s;
// }