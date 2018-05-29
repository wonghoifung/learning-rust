fn main() {
    fn  function            (i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;
    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    let one = || 1;
    println!("closure returning one: {}", one());

    use std::mem;
    let color = "green";
    let print = || println!("`color`: {}", color);
    print();
    print();
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();
    // let reborrow = &mut count;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume();

    let haystack = vec![1, 2, 3];
    // let contains = move |needle| haystack.contains(needle);
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    println!("There're {} elements in vec", haystack.len());
    // 9.2.2
}



