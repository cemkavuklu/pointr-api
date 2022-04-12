#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db;
use lib::model::{Building, Level, Site};
use rocket::http::{ContentType, RawStr, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/v1/sites",
        routes![
            get_sites,
            get_site,
            create_sites,
            delete_site,
            get_site_buildings,
            get_site_building,
            add_buildings_to_site,
            delete_building_from_site,
            add_levels_to_site_building
        ],
    )
}

// Retrieve all sites
#[get("/")]
fn get_sites() -> Json<Option<Vec<Site>>> {
    Json(db::read_sites())
}

// Retrieve a single site
#[get("/<site_name>")]
fn get_site(site_name: &RawStr) -> Json<Option<Site>> {
    Json(db::read_site(
        site_name.url_decode().expect("Failed to decode site name."),
    ))
}

// Import single site or multiple sites
#[post("/", data = "<sites>")]
fn create_sites(sites: Json<Vec<Site>>) -> ApiResponse {
    let is_created = db::insert_sites(sites.to_vec());

    match is_created {
        true => ApiResponse {
            json: json!({"status": "Import successful"}),
            status: Status::Created,
        },
        _ => ApiResponse {
            json: json!({"error": "Import failed"}),
            status: Status::UnprocessableEntity,
        },
    }
}

// Delete a site
#[delete("/<site_name>")]
fn delete_site(site_name: &RawStr) -> ApiResponse {
    let is_removed = db::delete_site(site_name.url_decode().expect("Failed to decode site name."));

    match is_removed {
        true => ApiResponse {
            json: json!({"status": "Delete successful"}),
            status: Status::Created,
        },
        _ => ApiResponse {
            json: json!({"error": "Building not found"}),
            status: Status::NotFound,
        },
    }
}

// Retrieve all buildings of a site
#[get("/<site_name>/buildings")]
fn get_site_buildings(site_name: &RawStr) -> Json<Option<Vec<Building>>> {
    Json(db::read_site_buildings(
        site_name.url_decode().expect("Failed to decode site name."),
    ))
}

// Retrieve a single building of a site
#[get("/<site_name>/buildings/<building_name>")]
fn get_site_building(site_name: &RawStr, building_name: &RawStr) -> Json<Option<Building>> {
    Json(db::read_site_building(
        site_name.url_decode().expect("Failed to decode site name."),
        building_name
            .url_decode()
            .expect("Failed to decode building name."),
    ))
}

// Add building to a site
#[post("/<site_name>/buildings", data = "<buildings>")]
fn add_buildings_to_site(site_name: &RawStr, buildings: Json<Vec<Building>>) -> ApiResponse {
    let is_created = db::add_buildings_to_site(
        site_name.url_decode().expect("Failed to decode site name."),
        buildings.to_vec(),
    );

    match is_created {
        true => ApiResponse {
            json: json!({"status": "Import successful"}),
            status: Status::Created,
        },
        _ => ApiResponse {
            json: json!({"error": "Import failed"}),
            status: Status::UnprocessableEntity,
        },
    }
}

// Remove a building from a site
#[delete("/<site_name>/buildings/<building_name>")]
fn delete_building_from_site(site_name: &RawStr, building_name: &RawStr) -> ApiResponse {
    let is_removed = db::remove_building_from_site(
        site_name.url_decode().expect("Failed to decode site name."),
        building_name
            .url_decode()
            .expect("Failed to decode building name."),
    );

    match is_removed {
        true => ApiResponse {
            json: json!({"status": "Delete successful"}),
            status: Status::Created,
        },
        _ => ApiResponse {
            json: json!({"error": "Building not found"}),
            status: Status::NotFound,
        },
    }
}

// Add single level or multiple levels to a building of a site
#[post("/<site_name>/buildings/<building_name>/levels", data = "<levels>")]
fn add_levels_to_site_building(
    site_name: &RawStr,
    building_name: &RawStr,
    levels: Json<Vec<Level>>,
) -> ApiResponse {
    let is_created = db::insert_levels_to_building(
        site_name.url_decode().expect("Failed to decode site name."),
        building_name
            .url_decode()
            .expect("Failed to decode building name."),
        levels.to_vec(),
    );

    match is_created {
        true => ApiResponse {
            json: json!({"status": "Import successful"}),
            status: Status::Created,
        },
        _ => ApiResponse {
            json: json!({"error": "Import failed"}),
            status: Status::UnprocessableEntity,
        },
    }
}
