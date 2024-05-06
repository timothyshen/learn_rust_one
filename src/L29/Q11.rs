/* Fill in the blank and fix the error*/
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32>{

    let num = 5;

    if x > 1{
        move |x| x + num
    } else {
        move |x| x + num
    }
}

