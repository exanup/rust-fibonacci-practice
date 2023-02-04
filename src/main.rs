use core::ops::Range;
use std::collections::HashMap;

const FIBO_INDEX_MAX: u8 = 185;

fn main() {
    const FIBO_RANGE: Range<u8> = 0..200;

    for i in FIBO_RANGE {
        println!("loop: fibo({i}) => {:?}", get_nth_fibonacci_loop(i));
    }

    for i in FIBO_RANGE {
        println!(
            "tail call: fibo({i}) => {:?}",
            get_nth_fibonacci_tail_call(i)
        );
    }

    for i in FIBO_RANGE {
        println!("dynamic: fibo({i}) => {:?}", get_nth_fibonacci_dynamic(i));
    }

    for i in FIBO_RANGE {
        println!("naive: fibo({i}) => {:?}", get_nth_fibonacci_naive(i));
    }
}

fn get_nth_fibonacci_naive(n: u8) -> Result<u128, &'static str> {
    const FIBO_NAIVE_INDEX_MAX: u8 = 40;

    fn get_nth_fibonacci_naive_helper(n: u8) -> u128 {
        match n {
            0 => 0,
            1 => 1,
            n => get_nth_fibonacci_naive_helper(n - 1) + get_nth_fibonacci_naive_helper(n - 2),
        }
    }

    if n > FIBO_INDEX_MAX {
        return Err("Overflow");
    }

    if n > FIBO_NAIVE_INDEX_MAX {
        return Err("Too slow to calculate");
    }

    Ok(get_nth_fibonacci_naive_helper(n))
}

fn get_nth_fibonacci_loop(n: u8) -> Result<u128, &'static str> {
    if n > FIBO_INDEX_MAX {
        return Err("Overflow");
    }

    let mut a = 0;
    let mut b = 1;

    for _i in 0..n {
        (a, b) = (b, a + b);
    }

    Ok(a)
}

fn get_nth_fibonacci_tail_call(n: u8) -> Result<u128, &'static str> {
    fn get_nth_fibonacci_tail_call_helper(n: u8, a: u128, b: u128) -> u128 {
        match n {
            0 => a,
            n => get_nth_fibonacci_tail_call_helper(n - 1, b, a + b),
        }
    }

    if n > FIBO_INDEX_MAX {
        return Err("Overflow");
    }

    Ok(get_nth_fibonacci_tail_call_helper(n, 0, 1))
}

fn get_nth_fibonacci_dynamic(n: u8) -> Result<u128, &'static str> {
    fn get_nth_fibonacci_dynamic_helper(fibonacci_cache: &mut HashMap<u8, u128>, n: u8) -> u128 {
        match n {
            0 => 0,
            1 => 1,
            n => {
                let fibo_n_minus_1 = match fibonacci_cache.get(&(n - 1)).copied() {
                    Some(val) => val,
                    None => {
                        let val = get_nth_fibonacci_dynamic_helper(fibonacci_cache, n - 1);
                        fibonacci_cache.insert(n - 1, val);

                        val
                    }
                };

                let fibo_n_minus_2 = match fibonacci_cache.get(&(n - 2)).copied() {
                    Some(val) => val,
                    None => {
                        let val = get_nth_fibonacci_dynamic_helper(fibonacci_cache, n - 2);
                        fibonacci_cache.insert(n - 2, val);

                        val
                    }
                };

                fibo_n_minus_1 + fibo_n_minus_2
            }
        }
    }

    let mut fibonacci_cache = HashMap::new();

    if n > FIBO_INDEX_MAX {
        return Err("Overflow");
    }

    Ok(get_nth_fibonacci_dynamic_helper(&mut fibonacci_cache, n))
}
