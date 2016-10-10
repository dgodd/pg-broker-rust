extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, MediaType};
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;
use std::collections::hash_map::HashMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Service {
    id: String,
    name: String,
    description: String,
    bindable: bool,
    tags: Vec<String>,
    metadata: ServiceMetadata,
    plans: Vec<ServicePlan>,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct ServiceMetadata {
    displayName: String,
    imageUrl: String,
    longDescription: String,
    providerDisplayName: String,
    documentationUrl: String,
    supportUrl: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ServicePlan {
    id: String,
    name: String,
    description: String,
    metadata: ServicePlanMetadata,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct ServicePlanMetadata {
    bullets: Vec<String>,
    costs: Vec<PlanCost>,
    displayName: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PlanCost {
    amount: HashMap<String, f32>,
    unit: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Catalog {
    services: Vec<Service>,
}


fn read_catalog(path: &str) -> Result<Catalog, String> {
    let mut f = try!(File::open(path).map_err(|_| "Could not open file"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s).map_err(|_| "Could not read file"));
    let mut catalog: Catalog = try!(json::decode(&s).map_err(|_| "Coud not parse json from file"));
    for mut service in catalog.services.iter_mut() {
        service.id = "hi mom".to_string();
        for mut plan in service.plans.iter_mut() {
            plan.id = "hi dad".to_string();
        }
    }
    Ok(catalog)
}

fn main() {
    let mut server = Nickel::new();
    let catalog = read_catalog("catalog.json").unwrap();
    let catalog_json = json::encode(&catalog).unwrap();

    server.get("/v2/catalog",
               middleware! { |_, mut res|
        res.set(MediaType::Json);
        catalog_json.clone()
    });

    let listening = server.listen("127.0.0.1:6767").expect("Failed to launch server");
    println!("Listening on: {:?}", listening.socket());
}
