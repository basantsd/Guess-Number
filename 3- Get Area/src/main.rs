use colored::Colorize;
use std::{i32, io};

struct AreaParameter {
    atype: i32,
    width: f32,
    height: f32,
}

impl AreaParameter {
    fn area(&self) -> f32 {
        match self.atype {
            1 => self.width * self.height,
            2 => 3.14 * (self.width * self.width),
            3 => self.width * self.width,
            4 => 0.5 * (self.width * self.height),
            _ => 3.14 * (self.width * self.height)
            
        }
    }
}

fn main() {
    println!("{}", "|| ****** Type To Get Area ****** ||".yellow());
    println!("For Rectangle {}", "'1' (default)".red());
    println!("For Circle {}", "'2'".red());
    println!("For Square {}", "'3'".red());
    println!("For Triangle {}", "'4'".red());
    println!("For Ellipse {}", "'5'".red());

    let mut get_type = String::new();
    io::stdin().read_line(&mut get_type).expect("1");

    if get_type.trim() == "" {
        get_type = "1".to_string();
    }

    println!("{}", "========================================".green());
    println!("Ok You Select : {}", get_type.red());
    println!("{}", "========================================".green());
    getinput(get_type.trim().parse().unwrap());

    let mut inputdata = String::new();
    io::stdin().read_line(&mut inputdata).expect("none");

    if inputdata.trim() == "" {
        inputdata = "1,1".to_string();
    }

    let data: Vec<&str> = inputdata.split(',').collect();

    let tt = AreaParameter {
        atype: get_type.trim().parse().unwrap(),
        width: data[0].trim().parse().unwrap(),
        height: if data.len() == 2 { data[1].trim().parse().unwrap() } else { 1.0 },
    };

    println!("Area : {}", tt.area());
}

fn getinput(atype: i32) {
    match atype {
        1 => println!("{}", "Enter length,width".yellow().bold()),
        2 => println!("{}", "Enter radius of circle".yellow().bold()),
        3 => println!("{}", "Enter sides of square".yellow().bold()),
        4 => println!("{}", "Enter base,height".yellow().bold()),
        _ => println!("{}", "Enter radius of major axis,minor axis".yellow().bold()),
    }
}
