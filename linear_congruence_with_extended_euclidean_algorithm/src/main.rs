fn main() {
    let a = 5;
    let b = 3;
    let m = 8;
    
    if let Some(res) = linear_congruence(a, b, m) {
        println!("The solution is x ≡ {} (mod {})", res, m);
    } else {
        println!("No solution exists.");
    }
}

fn extended_euclids_algo(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (gcd, x1, y1) = extended_euclids_algo(b, a % b);

    // Calculate x and y
    let x = y1;
    let y = x1 - (a / b) * y1;

    (gcd, x, y)
}

fn linear_congruence(a:i32,b:i32,m:i32)->Option<i32>{
    // get gcd and co-efficients
    let (gcd,x,_y)=extended_euclids_algo(a,m);
    
    // if gcd does not divide b then there are no solutions
    if b%gcd!=0{
       return None;
    }
    let x0=(((x*b)%m)+m)%m;
   
    Some(x0)
}
