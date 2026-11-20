fn main() {
    println!("{}", find_fib(5))
}

fn find_fib(num : u32) -> u32 {
    let mut pointer_a = 0;
    let mut pointer_b = 1;

    if num == 1 {
        return pointer_a
    }

    if num == 2 {
        return pointer_b
    }

    for _ in 1..num-1 {
      let temp = pointer_b;
      pointer_b = pointer_a + pointer_b;
      pointer_a = temp;
    }
    return pointer_b;
}