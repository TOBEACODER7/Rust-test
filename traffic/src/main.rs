enum trafficLights {
    red,
    yellow,
    green,
}

trait traffic {
    fn time(&self) -> u8;
}

impl traffic for trafficLights {
    fn time(&self) -> u8 {
        match self {
            trafficLights::red => 60,
            trafficLights::yellow => 10,
            trafficLights::green => 60,
        }
    }
}

fn main() {
    let red_l = trafficLights::red;
    let yellow_l = trafficLights::yellow;
    let green_l = trafficLights::green;

    println!("red: {}", red_l.time());
    println!("yellow: {}", yellow_l.time());
    println!("green: {}", green_l.time());
}
