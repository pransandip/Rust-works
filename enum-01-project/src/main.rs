/* enum = enumeration = list of choices */
// struct = something AND something AND something
// enum = something OR something OR something -> making one choice

enum ThingsInTheSky {
    Sun,
    Starts,
}

fn create_skystate(time: u8) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Starts,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    // we pass it as a reference so it doesn't die
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Starts => println!("I can see the stars"),
    }
}

fn main() {
    let time = 19;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}
