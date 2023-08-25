# PLENO - DevOps Challenge

**Welcome to PLENO DevOps challenge!**

You will find 2 microservices built with Rust Actix-Web called

* Service1
* Service2

that play pingpong through a very simple `HTTP` request as follow:

* Service1 $\rightarrow$ GET method sends a request to `http://localhost:8081/pong`.
* Service2 $\rightarrow$ POST method sends the response "Pong from service2!" to `http://localhost:8080/ping`.

# How To

* I have given you the right to clone this repository. Afterwards, you need to push it to your own repository and share it with me.
* Don't forget to add or modify the codes related to the endpoints and CORS. These codes are written in:
  * Service 1 - `src/main.rs`
    * Line 19 - `match client.post("http://127.0.0.1:8081/pong").send().await`
    * Line 37 - `.allowed_origin("http://127.0.0.1:8081");`
  * Service 2 - `src/main.rs`
    * Line 18 - `.allowed_origin("http://127.0.0.1:8080");`

# The Tasks

The project structure is as follow:

```shell
DevOpsChallenge
├── .github/
    ├── ci.yaml             # Task 1: Create CI pipleine for Github Actions for dockerized service
├── microservice1/
    ├── src/
        ├── main.rs
    ├── Dockerfile          # Task 2: Create Docker container with 3 stages: builder, dev, and prod.
    ├── docker-compose.yaml # Task 3: Create a simple Docker Compose script to start the service 1 container.
    ├── k8.yaml             # Task 4: Create Kubernetes cluster for the dockerized service 1.
├── microservice2/
    ├── src/
        ├── main.rs
    ├── Dockerfile          # Task 5: Create Docker container with 3 stages: builder, dev, and prod.
    ├── docker-compose.yaml # Task 6: Create a simple Docker Compose script to start the service 2 container.
    ├── k8.yaml             # Task 7: Create Kubernetes cluster for the dockerized service 2.
├── docker-compose.yaml     # Task 8: Create the last Docker Compose script to start both containers at once for dev environment.
```

As mentioned in the project structure, your task is to implement the following:

* Task 1: Create CI pipleine for Github Actions for dockerized services: build, lint, and test.
* Task 2: Create Docker container with 3 stages: builder, dev, and prod.
* Task 3: Create a simple Docker Compose script to start the service 1 container.
* Task 4: Create Kubernetes cluster for the dockerized service 1.
* Task 5: Create Docker container with 3 stages: builder, dev, and prod.
* Task 6: Create a simple Docker Compose script to start the service 2 container.
* Task 7: Create Kubernetes cluster for the dockerized service 2.
* Task 8: Create the last Docker Compose script to start both containers at once for dev environment.

# The Objectives

For the challenge, I prepared a checklist for you to keep the overview of the objective for this challenge:

- [ ] You need to create a GitHub repository with the submission.
- [ ] Your package GhitHub Package Registry must contain the built Dockerized artefact from your CI.
- [ ] I need to have permission to observe the repo and to download the artefact for local testing.
- [ ] Containerize both services with the network `pleno-network`.
- [ ] Run both containers in a Kubernetes pod and let them communicate with a private network.
- [ ] Create a CI for the containerized applications that build, lint, and test the container.
- [ ] Pass the test in your own CI.

If you have more objectives that you want to show us within this challenge, you are welcome to do so!

# Setup Guide

### Start Server

To start both services start with the `service2` application:

```shell
$ cd service2
$ cargo install
$ cargo run
```

Once it is running got to `http://localhost:8081`. Then start another terminal and run

```shell
$ cd service1
$ cargo install
$ cargo run
```

Once both services are running, you will se the following message in each of the 2 endpoints:

* Service 1:
  * `http://localhost:8080/` => Hello world in service 1!
  * `http://localhost:8080/ping` => Pong from service2!
* Service 2:
  *  `http://localhost:8081/` => Hello world in service 2!

### Test Application

For testing, you just need to go into eachn service directoy and run the following:

```shell
cargo test
```

You will be seeing the following in your terminal:

```shell
❯ cargo test                   
   Compiling service2 v0.1.0 (/PATH/TO/service1)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/main.rs (target/debug/deps/service1-acf545c943eedeb7)

running 1 test
test tests::test_hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# Last Words

I hope you can enjoy the challenge since these kind of tasks will be your bread and butter and please do it like how you want to do it since there will be no right and wrong.

Happy engineering!

---
