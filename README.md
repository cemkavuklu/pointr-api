# Sample Rust API
This repo contains a Sample API written in Rust

You can find the OpenAPI specification for this API [here](sample-rust-api-spec.yaml).

The specification can be viewed with [Swagger Editor](https://editor.swagger.io/).

Simply copy and paste the contents of the spec file into the Swagger Editor.

## Table of Contents

- [Sample Rust API](#sample-rust-api)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Assumptions for the API](#assumptions-for-api)
  - [Running the API Server](#running-the-api-server)
  - [Endpoints](#endpoints)
    - [Base URL](#base-url)
    - [Import Sites](#import-sites)
    - [Retrieve all Sites](#retrieve-all-sites)
    - [Retrieve Existing Site](#retrieve-existing-site)
    - [Delete a Site](#delete-a-site)
    - [Import Buildings](#import-buildings)
    - [Retrieve All Buildings of a Site](#retrieve-all-buildings-of-a-site)
    - [Retrieve a Building of a Site](#retrieve-a-building-of-a-site)
    - [Delete a Building from a Site](#delete-a-building-from-a-site)
    - [Import a single level or multiple levels](#import-a-single-level-or-multiple-levels)
  - [Testing](#testing)

## Installation
The only requirement to run the API server is the [Rust Language Toolchain](https://www.rust-lang.org).

It can be installed from https://www.rust-lang.org/tools/install.

| :warning:        | It is essential for Windows users to install Visual Studio 2019 (at least Community Edition) with C++ Development Tools.       |
|---------------|:------------------------|

After installing the Rust Toolchain, just clone this repo to a folder of your choice. And you are ready to go.

## Assumptions for the API
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

### Base URL
`/v1/sites`

This is the base URL common to all endpoints in the API.
### Import Sites
`POST /`

Imports a single site or multiple sites.
```
$ curl --location --request POST 'http://localhost:8000/v1/sites/' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        "name": "North Wing",
        "buildings": [
            {
                "name": "Block 1",
                "levels": [
                    {
                        "name": "Level A"
                    },
                    {
                        "name": "Level B"
                    },
                    {
                        "name": "Level C"
                    },
                    {
                        "name": "Level D"
                    },
                    {
                        "name": "Level E"
                    }
                ]
            },
            {
                "name": "Block 2",
                "levels": [
                    {
                        "name": "Level A1"
                    }
                ]
            }
        ]
    }
]'
```

Returns response message with a status code of `HTTP 201` or `HTTP 422`.

Example Response for `HTTP 201`:
```
{
    "status": "Import successful"
}
```
### Retrieve all Sites
`GET /`

Retrieves all the sites in the database.

```
$ curl http://localhost:8000/v1/sites
```

Returns a JSON array of site objects.

Example Response:
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

### Retrieve Existing Site

`GET /<site_name>`

Retrieves a single site.
```
$ curl http://localhost:8000/v1/sites/West%20Wing
```

Returns a JSON object for a single site.

Example Response:
```
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
        },
        {
            "name": "New Block 1",
            "levels": [
                {
                    "name": "Level A"
                },
                {
                    "name": "Level B"
                },
                {
                    "name": "Level C"
                },
                {
                    "name": "Level D"
                },
                {
                    "name": "Level E"
                }
            ]
        },
        {
            "name": "New Block 2",
            "levels": [
                {
                    "name": "Level A1"
                }
            ]
        }
    ]
}
```

### Delete a Site

`DELETE /<site_name>`

Removes a building from a site.
```
$ curl --request DELETE 'localhost:8000/v1/sites/North Wing'
```
Returns response message with a status code of `HTTP 201` or `HTTP 404`

Example Response for `HTTP 201`:
```
{
    "status": "Delete successful"
}
```
Example Response for `HTTP 404`:
```
{
    "error": "Site not found"
}
```

### Import Buildings
`POST /<site_name>/buildings`

Imports a single building or multiple buildings into a site.

```
$ curl --location --request POST 'http://localhost:8000/v1/sites/West Wing/buildings' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        "name": "New Block 1",
        "levels": [
            {
                "name": "Level A"
            },
            {
                "name": "Level B"
            },
            {
                "name": "Level C"
            },
            {
                "name": "Level D"
            },
            {
                "name": "Level E"
            }
        ]
    },
    {
        "name": "New Block 2",
        "levels": [
            {
                "name": "Level A1"
            }
        ]
    }
]'
```
Returns response message with a status code of `HTTP 201` or `HTTP 422`

Example Response for `HTTP 201`:
```
{
    "status": "Import successful"
}
```

### Retrieve All Buildings of a Site
`GET /<site_name>/buildings`

List all buildings within a site.

```
$ curl --location --request GET 'localhost:8000/v1/sites/West Wing/buildings/'
```
Example Response:
```
[
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
```

### Retrieve a Building of a Site
`GET /<site_name>/buildings/<building_name>`

List a single building by name within a site.
```
$ curl --location --request GET 'localhost:8000/v1/sites/West Wing/buildings/Block A'
```
Example Response:
```
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
}
```

### Delete a Building from a Site

`DELETE /<site_name>/buildings/<building_name>`

Removes a building from a site.

```
$ curl --location --request DELETE 'http://localhost:8000/v1/sites/West Wing/buildings/New Block 2'
```
Returns response message with a status code of `HTTP 201` or `HTTP 404`

Example Response for `HTTP 201`:
```
{
    "status": "Delete successful"
}
```
Example Response for `HTTP 404`:
```
{
    "error": "Building not found"
}
```

### Import a single level or multiple levels

`POST /<site_name>/buildings/<building_name>/levels`

Imports single or multiple levels into a building of a site.

```
$ curl --location --request POST 'http://localhost:8000/v1/sites/West Wing/buildings/Block A/levels' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        "name": "New Level"
    }
]'
```
Returns response message with a status code of `HTTP 201` or `HTTP 422`

Example Response for `HTTP 201`:
```
{
    "status": "Import successful"
}
```

## Testing
All tests are automated via a [Postman Collection](Pointr-API.postman_collection.json).

You can download Postman from [here](https://www.postman.com/downloads/).

After the installation, in the Collections side tab, click on the `Import` button and upload the Collection's JSON file.

All requests in the collection have tests in their respective Tests tabs.

Also, all the test can be automatically run via [Postman's Collection Runner](https://learning.postman.com/docs/running-collections/intro-to-collection-runs/).
