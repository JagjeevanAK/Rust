mod struct_exp;
mod function;
mod enumaration;

fn main() {
    // // Integer 

    // let x:i32 =5;
    // println!("{}",x);

    // // String
    // let greeting = String::from("Hello World");
    // println!("{}",greeting);

    // // Accessing an single element

    // let char1 = greeting.chars().nth(0);
    // match char1{
    //     Some(c)=> println!("{}",c),
    //     None => println!("No charecter at the place you are tyring to access the element")
    // }

    // // Conditionals

    // let x:i32 = 31;

    // if x%2 == 0 {
    //     println!("Number is even");
    // } else{
    //     println!("Number is odd")
    // }


    // // Loops

    // for i in 1..101{
    //     println!("{}",i);
    // }

    let sentence = String::from("Hello Jagjeevan");

    let first_char = function::get_frist_char(&sentence);
    println!("{}",first_char);

    let frist_word = function::get_frist_word(sentence);
    println!("{}", frist_word);

    // // Ownership
    // let s2= &sentence;
    // println!("{} {}",sentence, s2)

    // // Borrowing mutable

    let mut s1 = String::from("Hello");
    function::update_str(&mut s1);
    println!("{}", s1);

    // Borrowing immutable

    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3= &s1;

    println!(" s1: {}\n s2: {}\n s3: {}",s1,s2,s3);

    let jagjeevan = struct_exp::User{
        active : true,
        username : String::from("jagjeevan@12"),
        email: String::from("jagjeevankashid97@gmail.com"),
        sign_in_count: 2
    };
    println!("User 1 username : {:?}", jagjeevan.username);

    let rect = struct_exp::React{
        width: 10,
        height:20
    };

    println!("\nArea of the rectangle is {}", rect.area())

    
}
