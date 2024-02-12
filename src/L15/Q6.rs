fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    // r被匹配成了一个值，r的值就是 v的可变引用，也就是&mut v
    // r的值，是v这个变量的可变引用地址， 2.3.2中的第一张图
    match r {
        value => value.push_str(" world!"),
    }

    //这里会报错，因为match已经拿走r的值的所有权
    println!("{}", r);
}

fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    // 这里r被解引用，获取的是v在堆上的数据
    // 因此需要用ref mut来获取堆上数据的可变引用
    match *r {
        ref mut value => value.push_str(" world!"),
    }

    // 这里不会报错，因为拿的是堆上的数据，对r 对v的所有权都没有影响
    println!("{}", r);
    println!("{}", v);
}