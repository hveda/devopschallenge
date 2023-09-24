# Devops Challenge

A foundational project to kickstart your journey into DevOps.

## Table of Contents

- [Devops Challenge](#devops-challenge)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Directory Structure](#directory-structure)
  - [Getting Started](#getting-started)
    - [Starting the Servers](#starting-the-servers)
    - [Starting the Servers without Docker](#starting-the-servers-without-docker)
  - [Test Application](#test-application)
  - [Considerations](#considerations)

## Introduction

This project consists of two services and utilizes Docker Compose as a local development setup. Continuous Integration and Continuous Deployment (CI/CD) are achieved through GitHub Actions. The CI/CD pipeline includes steps such as code testing, linting, Docker image building, and deployment to a Google Kubernetes Engine (GKE) cluster.

## Directory Structure

Your project directory structure should look like this:

```shell
.
├── .github
│   └── workflows
│       └── ci.yaml
├── .gitignore
├── README.md
├── docker-compose.yaml
├── dockerfile.d
│   ├── rust.dev.dockerfile
│   └── rust.dockerfile
├── service1
│   ├── Cargo.toml
│   ├── docker-compose.yaml
│   ├── k8s.yaml
│   └── src
│       └── main.rs
└── service2
    ├── Cargo.toml
    ├── docker-compose.yaml
    ├── k8s.yaml
    └── src
        └── main.rs
```

- `.github/workflows/ci.yaml`: This GitHub Actions workflow is designed for continuous integration.
- `.gitignore`: This file specifies which files and directories should be excluded from version control.
- `README.md`: You are currently reading this document.
- `docker-compose.yaml`: This file is your Docker Compose configuration, used to run the project in a local development environment.
- `dockerfile.d`: This directory contains Dockerfiles for various configurations. Dockerfiles are grouped in this directory to allow for the use of a single Dockerfile for multiple services that share similar build steps.
- `service1` and `service2`: These directories house your project's services. Each service includes its own `Cargo.toml` for managing Rust dependencies, a `docker-compose.yaml` for Docker Compose configurations, a `k8s.yaml` for Kubernetes configurations, and a `src` directory where the source code resides.


## Getting Started

### Starting the Servers
To launch both servers without rebuilding, use the following command:

```shell
docker-compose up
```

To start both servers after making code changes (for development purposes):

```shell
docker-compose up --build
```

To launch a single service, use one of the following commands:

```shell
docker-compose up service1
# to rebuild user param --build
docker-compose up --build service1
```
or

```shell
docker-compose up service2
# to rebuild user param --build
docker-compose up --build service2
```

Alternatively, to run a single service, navigate to the service's directory and use docker-compose up. For example:


```shell
cd service1
docker-compose up
```

### Starting the Servers without Docker
To initiate both services, begin with the service2 application:

```shell
$ cd service2
$ cargo install
$ cargo run
```

Once it's running, go to [http://localhost:8081](http://localhost:8081). Then, open another terminal and run:


```shell
$ cd service1
$ cargo install
$ cargo run
```

After both services are running, you'll find the following endpoints:

* Service 1:
  * [http://localhost:8080/](http://localhost:8080) => Hello world in service 1!
  * [http://localhost:8080/ping](http://localhost:8080/ping) => Pong from service2!
* Service 2:
  * [http://localhost:8081/](http://localhost:8081) => Hello world in service 2!


## Test Application

To run tests, navigate to each service's directory and execute the following:


```shell
cargo test
```

You'll see the test results displayed in your terminal:

```shell
❯ cargo test                   
   Compiling service2 v0.1.0 (/PATH/TO/service1)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/main.rs (target/debug/deps/service1-acf545c943eedeb7)

running 1 test
test tests::test_hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Considerations
Areas for enhancement or action:

- Implement hot-reloading of Docker images to accelerate local development.
- Set up support for multiple environments within Docker Compose.

Wishing you productive engineering!

---
