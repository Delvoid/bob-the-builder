#[derive(Copy, Clone, Debug)]
pub enum CarType {
    CityCar,
    SportsCar,
    Suv,
}

#[derive(Debug)]
pub enum Transmission {
    SingleSpeed,
    Manual,
    Automatic,
    SemiAutomatic,
}

pub struct Engine {
    volume: f64,
    mileage: f64,
    started: bool,
}

impl Engine {
    pub fn new(volume: f64, mileage: f64) -> Self {
        Self {
            volume,
            mileage,
            started: false,
        }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.started = false;
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn mileage(&self) -> f64 {
        self.mileage
    }

    pub fn go(&mut self, mileage: f64) {
        if self.started() {
            self.mileage += mileage;
        } else {
            println!("Cannot go(), you must start engine first!");
        }
    }
}

pub struct GpsNavigator {
    route: String,
}

impl GpsNavigator {
    pub fn new() -> Self {
        Self::from_route("123 abc road, Stoke-on-Trent to London Airport".into())
    }

    pub fn from_route(route: String) -> Self {
        Self { route }
    }

    pub fn route(&self) -> &String {
        &self.route
    }
}
