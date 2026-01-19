pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
}

fn lcm_helper(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn lcm(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(lcm_helper).unwrap()
}
