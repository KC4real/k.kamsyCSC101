fn main() {
    let fullname = "Kamsy Kasie Nwachukwu";
    let department = "software engineering";
    let uni = "pan atlantic university";

    let mut school = "School of science".to_string();
    //push string
    school.push_str("and technology");



   
    println!("My name is:{}", fullname );
     //check length 
    println!("the length my fullname is:{}",fullname.len());
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",uni);
}
