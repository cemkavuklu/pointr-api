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
* It is assumed that a building belongs to a site and cannot exist as a standalone entity. While this may not fit all use real life use cases, it is enough for demonstration purposes.
* The above assumption is also valid for a level. A level belongs to a building and cannot exist as a standalone entity.
  
In the light of above statements, the base URL for the API is `/v1/sites/`. Here `v1` denotes the semantic version.
