
// fn main() {
//     println!("Hello, world!");
// }

// use std::io;

fn main() {
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // println!("You guessed: {}", guess);

    let random_variable = 3.5;
    let character = "💎";

    let large_number = 500000;
    println!("{}\n{}\n{}", random_variable, large_number, character);
    // println!("{}", character);

    let mut fees: f32 = 35000.0;
    println!("fees - {}", fees);
    fees = 343.223;
    println!("modified fees - {}", fees);


    // String
    // String literal

    let company_name: &str = "Tutorial Point";
    println!("{}", company_name);

    // String Object
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let content_string = String::from("TutorialPoint");
    println!("length is {}", content_string.len());

    // Utilize String Methods

    let mut test_string = String::from("12");
    println!("original test_string - {}", test_string); 
    test_string.push('0');
    println!("pushed 0 - {}", test_string);
    test_string.push_str("000");
    println!("pushed 000 - {}", test_string);
    let replaced_string = test_string.replace("0", "9");
    println!("replace 0 to 9 - {}", replaced_string);
    test_string = "Tutorials Point \r\n".to_string();
    println!("test_string - {}", test_string.len());

    test_string = "Satoru,Japan,rust".to_string();
    for token in test_string.split(",") {
        println!("token is {}", token);
    }

    for ch in test_string.chars() {
        println!("{}", ch)
    }

    // Concatnation of String with  + operation
    let n1 = "Tutorials".to_string();
    let n2 =  "Points".to_string();

    let n3 = n1 + &n2;
    println!("{}", n3);

    // Concatenation with macro function
    let n1 = "New Tutorials".to_string();
    let n3 = format!("{} {}", n1, n2);
    println!("{}", n3);
}