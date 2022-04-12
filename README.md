# Pointr API
This repo contains an API for the Pointr Case Study.

## Installation
The only requirement to run the API server is the [Rust Language Toolchain](https://www.rust-lang.org).

It can be installed from https://www.rust-lang.org/tools/install.

| :warning:        | It is essential for Windows users to install Visual Studio 2019 (at least Community Edition) with C++ Development Tools.       |
|---------------|:------------------------|

After installing the Rust Toolchain, just clone this repo to a folder of your choice. And you are ready to go.

## Assumptions for the Case Study
All endpoints in this API are related to `sites` at the lowest level. The reasoning behind this is as follows:
* It is assumed that a building belongs to a site and cannot exist as a standalone entity. While this may not fit all real life use cases, it is enough for demonstration purposes.
* The above assumption is also valid for a level. A level belongs to a building and cannot exist as a standalone entity.
  
In the light of above statements, the base URL for the API is `/v1/sites/`. Here `v1` denotes the semantic version.

This project uses a plain JSON file as a mock database. This is done simply to reduce the development time of the server. 

## Running the API Server
To run the API Server, simply navigate into the folder you cloned the repo into, and run the following command in a terminal window (without the `$` symbol):
```
$ cargo run
```

This will build all the necessary dependencies and start the local server. After this step you should be able to see the following message in your terminal:
```
...
Mounting /v1/sites:
    => GET /v1/sites (get_sites)
    => GET /v1/sites/<name> (get_site)
    => POST /v1/sites (create_sites)
    => DELETE /v1/sites/<site_name> (delete_site)
    => GET /v1/sites/<site_name>/buildings (get_site_buildings)
    => GET /v1/sites/<site_name>/buildings/<building_name> (get_site_building)
    => POST /v1/sites/<site_name>/buildings (add_buildings_to_site)
    => DELETE /v1/sites/<site_name>/buildings/<building_name> (delete_building_from_site)
    => POST /v1/sites/<site_name>/buildings/<building_name>/levels (add_levels_to_site_building)
🚀 Rocket has launched from http://localhost:8000
```

Now you are ready to send some requests to the server!

## Endpoints
### Retrieve all Sites
`GET /v1/sites`

Retrieves all the sites in the database.
```
$ curl http://localhost:8000/v1/sites
```

Returns a JSON array of site objects

Example:
```
[
    {
        "name": "West Wing",
        "buildings": [
            {
                "name": "Block A",
                "levels": [
                    {
                        "name": "Level 1"
                    },
                    {
                        "name": "Level 2"
                    },
                    {
                        "name": "Level 3"
                    },
                    {
                        "name": "Level 4"
                    },
                    {
                        "name": "Level 5"
                    }
                ]
            },
            {
                "name": "Block B",
                "levels": [
                    {
                        "name": "Level 1"
                    }
                ]
            }
        ]
    },
    {
        "name": "East Wing",
        "buildings": [
            {
                "name": "Block B",
                "levels": [
                    {
                        "name": "Level 1"
                    },
                    {
                        "name": "Level 2"
                    },
                    {
                        "name": "Level 3"
                    }
                ]
            }
        ]
    }
]
```
