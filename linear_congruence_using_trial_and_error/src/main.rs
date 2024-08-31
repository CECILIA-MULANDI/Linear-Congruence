fn main() {
    // println!("The gcd is:{:?}", extended_euclids_algo(99,78));
    println!("{:?}", linear_congruence(5, 3, 8));
}

fn euclids_algo(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// this uses trial and error method
fn linear_congruence(a: i32, b: i32, modulo: i32) -> Vec<i32> {
    let gcd = euclids_algo(a, modulo);
    let mut results = Vec::new();
    if b % gcd == 0 {
        println!("There are {gcd} solutions");
        for i in 0..modulo {
            if (a * i) % modulo == b {
                results.push(i);
            }
        }
    }
    else{
        println!("There are no solutions");
    }
    results
}
