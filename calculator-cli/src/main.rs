use std::io;

use colored::Colorize;

fn main() {
    println!("{}","==============================\n SUM OF NUMBER \n ==============================".red());
    println!("Sum of x & y : {}", sumnum().to_string().green());

    println!("{}","==============================\n SUBTRACTION OF NUMBER \n ==============================".red());
    println!("Subtraction of x & y : {}", subtraction().to_string().green());

    println!("{}","==============================\n multiplication OF NUMBER \n ==============================".red());
    println!("multiplication of x & y : {}", multiplication().to_string().green());

    println!("{}","==============================\n division OF NUMBER \n ==============================".red());
    println!("division of x & y : {}", division().to_string().green());

}

fn sumnum() -> i32 {
    let mut xnum = String::new();
    let mut ynum = String::new();
    println!("{}", "Enter Value Of x : ");
    io::stdin()
        .read_line(&mut xnum)
        .expect("Please Enter Number");
    println!("{}", "Enter Value Of y : ");
    io::stdin()
        .read_line(&mut ynum)
        .expect("Please Enter Number");

    let xn: i32 = match xnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let yn: i32 = match ynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return xn + yn;
}



fn subtraction() -> i32 {
    let mut xnum = String::new();
    let mut ynum = String::new();
    println!("{}", "Enter Value Of x : ");
    io::stdin()
        .read_line(&mut xnum)
        .expect("Please Enter Number");
    println!("{}", "Enter Value Of y : ");
    io::stdin()
        .read_line(&mut ynum)
        .expect("Please Enter Number");

    let xn: i32 = match xnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let yn: i32 = match ynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return if xn > yn { xn-yn } else { 0 };
}


fn multiplication() -> i32 {
    let mut xnum = String::new();
    let mut ynum = String::new();
    println!("{}", "Enter Value Of x : ");
    io::stdin()
        .read_line(&mut xnum)
        .expect("Please Enter Number");
    println!("{}", "Enter Value Of y : ");
    io::stdin()
        .read_line(&mut ynum)
        .expect("Please Enter Number");

    let xn: i32 = match xnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let yn: i32 = match ynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return xn*yn ;
}

fn division()->i32{
    let mut xnum = String::new();
    let mut ynum = String::new();
    println!("{}", "Enter Value Of x : ");
    io::stdin()
        .read_line(&mut xnum)
        .expect("Please Enter Number");
    println!("{}", "Enter Value Of y : ");
    io::stdin()
        .read_line(&mut ynum)
        .expect("Please Enter Number");

    let xn: i32 = match xnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let yn: i32 = match ynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return if xn > yn { xn/yn } else { 0 };
}