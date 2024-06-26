/* Fill in the blank using two approaches,
 and fix the error */
fn create_fn() -> impl fn(i32) -> i32 {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}