use std::time::Instant;

use sajl::Logger;
use serde::Serialize;

#[derive(Serialize, Debug)]
enum Items {
    Apple,
    Ipod,
    Puter,
    Steak,
}

#[derive(Serialize, Debug)]
enum ToyotaModel {
    Rav3,
    Camry,
}

#[derive(Serialize, Debug)]
enum TeslaModel {
    Model3,
    ModelS,
}

#[derive(Serialize, Debug)]
enum Car {
    Toyota(ToyotaModel),
    Tesla(TeslaModel),
}

#[derive(Serialize, Debug)]
struct Child {
    toy: String,
    age: usize,
}
#[derive(Serialize, Debug)]
struct User {
    name: String,
    age: usize,
    items: Vec<Items>,
    children: Vec<Child>,
    car: Car,
}

#[tokio::main]
async fn main() {
    let logger = Logger::new(None);

    let user = User {
        name: "Jose".to_string(),
        age: 43,
        items: vec![Items::Ipod, Items::Steak],
        car: Car::Tesla(TeslaModel::ModelS),
        children: vec![Child {
            age: 12,
            toy: "beans".to_string(),
        }],
    };

    let start = Instant::now();
    for _ in 0..100 {
        logger.error(&user);
    }

    println!("FOR LOOP DONE SENDING {:?}", start.elapsed())
    //
}
