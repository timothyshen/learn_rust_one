
// // 修复错误，不要新增或删除代码行
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in names {
//         // do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     }
    
//     println!("{:?}", numbers);
// } 








// 填空
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30)
}