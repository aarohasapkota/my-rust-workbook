pub struct Person{
    name: String,
    age: i32,
    children: i32
}


pub enum Color{
    Red,
    Green,
    Blue
}


impl Person{
    pub fn print(&self){
        println!("Name: {}, Age: {}, Children: {}",self.name, self.age, self.children); 
    }
    pub fn is_under_age(&self){
        if self.age < 18 {
            println!("{} is under age", self.name);
        } else {
            println!("{} is not under age", self.name);
        }
    }
} 

fn main() {
    println!("Hello, world!");
    let p: Person = Person{name: "John".to_string(), age: 30, children: 2};
    p.print();
    p.is_under_age();

    let c: Color = Color::Red;

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue")
    }


}
