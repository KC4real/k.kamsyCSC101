fn main() {
    let p :f64 = 520000000.00;
    let n :f64 = 5.00;
    let R :f64 = 10.00;
    // compound intrest 
    let A = p * (1.0+(R/100.0));
    println!("amount is {}",A);
    let ci = A - p;
    println!("compound intrest is {}",ci);
}