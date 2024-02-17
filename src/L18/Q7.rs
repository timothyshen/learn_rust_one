// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

// fn sum<T>(a: T, b: T) -> T
//     where T: std::ops::Add<Output=T>
// {
//     a + b
// }