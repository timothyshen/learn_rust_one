/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;

    // `inc` closure captures `count` by mutable reference, therefore count do not change during the call

    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 

    assert_eq!(count, 0);
}