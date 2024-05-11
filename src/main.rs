use std::io;

fn main() {
    println!("Temperature converter");

    loop {
        println!("====\n");
        println!("What do you want to convert?");
        println!("Press f for Celsius to Fahrenheit");
        println!("Press c for Fahrenheit to Celsius");
        println!("Press q to exit");
        let mut direction = String::new();

        io::stdin()
            .read_line(&mut direction)
            .expect("Cannot read the input");

        match direction.trim() {
            "q" => {
                println!("Goodbye!");
                break;
            },
            "f" => {
                let t = get_user_input();
                println!("{} deg Celsius = {} deg Fahrenheit", t, c_to_f(t))
            },
            "c" => {
                let t = get_user_input();
                println!("{} deg Fahrenheit = {} deg Celsius", t, f_to_c(t))
            },
            _ => {
                println!("Wrong input!");
                continue;
            }
        }
    }
}

fn get_user_input() -> i32 {
    let mut t = String::new();
    loop {
        println!("Please input a temperature value to convert:");
        io::stdin()
            .read_line(&mut t)
            .expect("Cannot read the input");

        match t.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
}

fn c_to_f(val: i32) -> i32 {
    val * 9/5 + 32
}

fn f_to_c(val: i32) -> i32 {
    (val - 32) * 5/9
}
