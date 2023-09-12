use std::thread::sleep;
use std::time::Duration;
use std::io;

fn main() {
    println!("Pomodoro RS");
    
    println!("Input your desired work time in minuts:");
    let mut ptimemin: i32 = input_to_int();
    let mut ptimesec = 0;

    println!("Input your desired break time in minuts:");
    let mut pbtimemin: i32 = input_to_int();

    while ptimemin >= 0 {
        if ptimesec == 0 {
            println!("{:02}:{:02}", ptimemin, ptimesec);
            sleep(Duration::from_secs(1));
            ptimesec = 59;
            ptimemin -= 1;
        } else {
            println!("{:02}:{:02}", ptimemin, ptimesec);
            ptimesec -= 1;
            sleep(Duration::from_secs(1));
        }
    }

    while pbtimemin >= 0 {
        if ptimesec == 0 {
            println!("{:02}:{:02}", pbtimemin, ptimesec);
            sleep(Duration::from_secs(1));
            ptimesec = 59;
            pbtimemin -= 1;
        } else {
            println!("{:02}:{:02}", pbtimemin, ptimesec);
            ptimesec -= 1;
            sleep(Duration::from_secs(1));
        }
    }

    println!("End of session...");
}

fn input_to_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Invalid input, please enter a valid integer.")
}
