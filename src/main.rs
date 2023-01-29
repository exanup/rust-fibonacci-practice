use core::ops::Range;
use std::collections::HashMap;

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
    fn get_nth_fibonacci_naive_helper(n: u8) -> u128 {
        if n == 0 {
            0
        } else if n == 1 || n == 2 {
            1
        } else {
            get_nth_fibonacci_naive_helper(n - 1) + get_nth_fibonacci_naive_helper(n - 2)
        }
    }

    if n > 185 {
        Err("Overflow")
    } else if n > 42 {
        Err("Too slow to calculate")
    } else {
        Ok(get_nth_fibonacci_naive_helper(n))
    }
}

fn get_nth_fibonacci_loop(n: u8) -> Result<u128, &'static str> {
    if n > 185 {
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
        if n == 0 {
            a
        } else {
            get_nth_fibonacci_tail_call_helper(n - 1, b, a + b)
        }
    }

    if n > 185 {
        Err("Overflow")
    } else {
        Ok(get_nth_fibonacci_tail_call_helper(n, 0, 1))
    }
}

fn get_nth_fibonacci_dynamic(n: u8) -> Result<u128, &'static str> {
    fn get_nth_fibonacci_dynamic_helper(fibonacci_cache: &mut HashMap<u8, u128>, n: u8) -> u128 {
        if n == 0 {
            return 0;
        }

        if n == 1 || n == 2 {
            return 1;
        }

        let fibo_n_minus_1 = match fibonacci_cache.get(&(n - 1)) {
            Some(val) => *val,
            None => {
                let val = get_nth_fibonacci_dynamic_helper(fibonacci_cache, n - 1);
                fibonacci_cache.insert(n - 1, val);

                val
            }
        };

        let fibo_n_minus_2 = match fibonacci_cache.get(&(n - 2)) {
            Some(val) => *val,
            None => {
                let val = get_nth_fibonacci_dynamic_helper(fibonacci_cache, n - 2);
                fibonacci_cache.insert(n - 2, val);

                val
            }
        };

        fibo_n_minus_1 + fibo_n_minus_2
    }

    let mut fibonacci_cache = HashMap::new();

    if n > 185 {
        Err("Overflow")
    } else {
        Ok(get_nth_fibonacci_dynamic_helper(&mut fibonacci_cache, n))
    }
}
