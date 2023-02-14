fn main() {
    let moon = 384400.0;
    println!("月までの距離は{}キロメートルです", moon);

    let car_speed: f64 = 80.0; // km/h
    let express_train_speed: f64 = 300.0; // km/h

    let car_time = moon / car_speed / 24.0;
    println!("車で行くと{}日かかります", car_time);

    let express_train_time = moon / express_train_speed / 24.0;
    println!("新幹線で行くと{}日かかります", express_train_time);
}