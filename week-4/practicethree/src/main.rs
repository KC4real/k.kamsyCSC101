fn main(){
    let name1 = "kamsy kasie";
    println!("My Name is {}",name1);
    //find and replace 
    let name2 = name1.replace("kamsy","kasie");
    println!("you can also call me {}",name2);
    let faculty = "Faculty of Science and technology";


    //find and replace 
    let school = faculty .replace("Faculty", "School");
    println!("i am a student of the {}", school);
}