#[macro_use]
extern crate lazy_static;

mod models;
mod controllers;

use actix_web::{ HttpServer, App, web::Data };
use actix_cors::Cors;

use controllers::use_graphql;
use models::graphql::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🦋 Seele is starting...");   
    
    let schema = create_schema();

    println!("🦋 Seele is running...");

    HttpServer::new(move ||
        App::new()
            .wrap(Cors::permissive().allow_any_origin())
            .app_data(Data::new(schema.clone()))
            .configure(use_graphql)
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}








// use serde_json::{Number, Value};
// use serde_derive::{Deserialize, Serialize};
// // use std::fs;
// // use std::path::Path;

// #[derive(Deserialize, Serialize, Debug)]
// struct Food {
//     id: u32,
//     name: String,
//     missy_comment: String,
// }

// #[derive(Deserialize, Serialize, Debug)]
// struct Schedule {
//     date: i64,
//     quantity: f64,
//     food: u32,
//     missy_grumpiness: u32,
// }

// #[derive(Deserialize, Serialize, Debug)]
// struct MissyFoodSchedule {
//     food: Vec<Food>,
//     missy_food_schedule: Vec<Schedule>,
// }



// fn main() {

//     // let sales_and_products = {
//     //     let file_content = fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
//     //     serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
//     // };
//     // println!("{:?}", serde_json::to_string_pretty(&sales_and_products).expect("LogRocket: error parsing to JSON"));



//     let input_path = "./data/missy_secrets.json";
//     let output_path = "./data/miss_secrets_dynamic.json";

//     let mut missy_diet = {
//         // Load the first file into a string.
//         let text = std::fs::read_to_string(&input_path).unwrap();

//         // Parse the string into a dynamically-typed JSON structure.
//         serde_json::from_str::<Value>(&text).unwrap()
//     };

//     // Get the number of elements in the object 'missy_food_schedule'
//     let nb_elements = missy_diet["missy_food_schedule"].as_array().unwrap().len();

//     for index in 0..nb_elements{
//         if let Value::Number(n) = &missy_diet["missy_food_schedule"][index]["quantity"] {
//             // Double the quantity for each element in 'missy_food_schedule'
//             missy_diet["missy_food_schedule"][index]["quantity"] =
//                 Value::Number(Number::from_f64(n.as_f64().unwrap() * 2.).unwrap());
//         }
//     }

//     // Save the JSON structure into the other file.
//     std::fs::write(
//         output_path,
//         serde_json::to_string_pretty(&missy_diet).unwrap(),
//     )
//     .unwrap();
// }