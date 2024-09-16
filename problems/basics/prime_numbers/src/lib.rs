#![forbid(unsafe_code)]

pub fn get_n_prime_numbers(n: u32) -> Vec<u32> {
    let mut ans: Vec<u32> = vec![];
    if n == 0 {
        return ans;
    }

    ans.push(2);
    let mut cur = 3;
    let mut k = ans.len() as u32;

    while k < n {
        let mut is_prime = true;
        for prime in &ans {
            if cur % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            ans.push(cur);
            k += 1;
        }

        cur += 2;
    }

    ans
}
