
// // 修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }

// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }


// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One, Number1::One);
//     assert_eq!(Number1::One, Number2::One);
// } 



// // 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1 = Message::Move{x:1, y:2}; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write(String::from("hello, world!")); // 使用 "hello, world!" 来初始化
// } 


// 仅填空并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::Move{x: 1, y: 2};

//     if let Message::Move{x:a, y:b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不要让这行代码运行！");
//     }
// } 


// 填空，并修复错误
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// } 

// fn show_message(msg: Message) {
//     println!("{}", msg);
// }



// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let n = six {
        println!("{}", n)
    } 
        
    panic!("不要让这行代码运行！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


