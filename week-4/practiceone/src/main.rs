fn main() {
    let name = "Kamsy kasie";
    let uni:&str = "Pan-atlantic university";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-lekki,Lagos";
    println!("name:{}",name );
    println!("Unicersity:{},\nAddres:{}",uni,addr);


    let department:&'static str ="computer Science";
    let school:&'static str ="school of science and technology ";
    println!("Department:{},\nSchool:{}",department,school);
}