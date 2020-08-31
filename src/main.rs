

fn main() {
    let state = 3;

    let error: RunningStates = match state {
        1 => {
            println!("The state is 1");
            println!("this is a new line");
            RunningStates::Ok
        },
        2 => {
            println!("2");
            RunningStates::UnexpectedValue(2)
        },
        x => RunningStates::LogicError(x)
    };

    match error {
        RunningStates::Ok => println!("All ok"),
        RunningStates::UnexpectedValue(y) => panic!("UNEXPECTED VALUE {}",y),
        RunningStates::LogicError(z) => println!("Got something strange, {}",z),
    }

}


pub enum RunningStates {
    Ok,
    UnexpectedValue(u8),
    LogicError(u8),

}

pub enum Ip {
    Ipv4(u8,u8,u8),
    Ipv6(u8,u8),
}
