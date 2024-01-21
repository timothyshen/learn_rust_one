
fn main1() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

// 不要修改 main 中的代码
fn main2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership2(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership2(s: String) -> String {
    let s1 = s;
    println!("{}", s1);
    return s1;
}



fn main3() {
    let s = give_ownership3();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership3() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    s
}


// 修复错误，不要删除任何代码行
fn main4() {
    let s = String::from("hello, world");

    print_str4(s.clone());

    println!("{}", s);
}

fn print_str4(s: String)  {
    println!("{}",s)
}

// 不要使用 clone，使用 copy 的方式替代
fn main5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

fn main6() {
    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world");

    println!("{}", s1);
}

fn main7() {
    let x = Box::new(5);

    let mut y = Box::new(4);     // 完成该行代码，不要修改其它行！

    *y = 4;

    assert_eq!(*x, 5);
}

fn main8() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;


   // 仅修改下面这行代码，且不要使用 `_s`
   println!("{:?}", t.1);
}

fn main() {
   let t = (String::from("hello"), String::from("world"));

   // 填空，不要修改其它代码
   let (s1, s2) = t.clone();

   println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}