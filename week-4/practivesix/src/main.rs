fn main() {
    let n1 = "Electronical".to_string();
    let n2 = "eletronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 reference is passed 

    //About Electrical/Electronic
    println!("\nThe {} is informed by the aspiration to train eletronic/electrical Engineering professionals in the areas of design, building and mainteance of electrical control systems,", n4);
    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;   // w2 reference is passed 
    println!();
    println!("{} is aimed at the development comppetent,creation of innovative minds",w3);
}
