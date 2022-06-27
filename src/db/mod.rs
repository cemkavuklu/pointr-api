use std::fs;

use crate::model::{Building, Level, Site};

static SITES_DB: &str = "data/sites.json";

fn _sites() -> Result<Vec<Site>, serde_json::Error> {
    let data = fs::read_to_string(SITES_DB).expect("Error reading from file!");
    let sites: Result<Vec<Site>, serde_json::Error> = serde_json::from_str(&data);
    sites
}

pub fn read_sites() -> Option<Vec<Site>> {
    match _sites() {
        Ok(sites) => Some(sites),
        Err(_) => None,
    }
}

fn _write_sites(sites: Vec<Site>) {
    let data = serde_json::to_string(&sites).expect("Failed to encode string into JSON");
    fs::write(SITES_DB, data).expect("Error saving file.");
}

pub fn read_site(name: String) -> Option<Site> {
    match _sites() {
        Ok(sites) => {
            let index = sites.iter().position(|s| s.name == name);

            match index {
                Some(i) => Some(sites[i].clone()),
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub fn insert_sites(site_list: Vec<Site>) -> bool {
    match _sites() {
        Ok(mut sites) => {
            for site in site_list {
                sites.push(site.clone());
            }
            _write_sites(sites);
            true
        }
        Err(_) => false,
    }
}

pub fn delete_site(name: String) -> bool {
    match _sites() {
        Ok(mut sites) => {
            let index = sites.iter().position(|s| s.name == name);

            match index {
                Some(i) => {
                    sites.remove(i);
                    _write_sites(sites);
                    true
                }
                None => false,
            }
        }
        Err(_) => false,
    }
}

pub fn read_site_buildings(site_name: String) -> Option<Vec<Building>> {
    match _sites() {
        Ok(sites) => {
            let index = sites.iter().position(|s| s.name == site_name);

            match index {
                Some(i) => Some(sites[i].buildings.clone()),
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub fn read_site_building(site_name: String, building_name: String) -> Option<Building> {
    match _sites() {
        Ok(sites) => {
            let index = sites.iter().position(|s| s.name == site_name);

            match index {
                Some(i) => {
                    let building_index = sites[i]
                        .buildings
                        .iter()
                        .position(|b| b.name == building_name);

                    match building_index {
                        Some(j) => Some(sites[i].buildings[j].clone()),
                        None => None,
                    }
                }
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub fn add_buildings_to_site(site_name: String, building_list: Vec<Building>) -> bool {
    match _sites() {
        Ok(mut sites) => {
            let index = sites.iter().position(|s| s.name == site_name);

            match index {
                Some(i) => {
                    for building in building_list {
                        sites[i].buildings.push(building);
                    }
                    _write_sites(sites);
                    true
                }
                None => false,
            }
        }
        Err(_) => false,
    }
}

pub fn remove_building_from_site(site_name: String, building_name: String) -> bool {
    match _sites() {
        Ok(mut sites) => {
            let index = sites.iter().position(|s| s.name == site_name);

            match index {
                Some(i) => {
                    let building_index = sites[i]
                        .buildings
                        .iter()
                        .position(|b| b.name == building_name);

                    match building_index {
                        Some(j) => {
                            sites[i].buildings.remove(j);
                            _write_sites(sites);
                            true
                        }
                        None => false,
                    }
                }
                None => false,
            }
        }
        Err(_) => false,
    }
}

pub fn insert_levels_to_building(
    site_name: String,
    building_name: String,
    level_list: Vec<Level>,
) -> bool {
    match _sites() {
        Ok(mut sites) => {
            let index = sites.iter().position(|s| s.name == site_name);

            match index {
                Some(i) => {
                    let building_index = sites[i]
                        .buildings
                        .iter()
                        .position(|b| b.name == building_name);

                    match building_index {
                        Some(j) => {
                            for level in level_list {
                                sites[i].buildings[j].levels.push(level);
                            }
                            _write_sites(sites);
                            true
                        }
                        None => false,
                    }
                }
                None => false,
            }
        }
        Err(_) => false,
    }
}
