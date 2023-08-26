# Setup Guide

## Start Server

To start both server without rebuild:
```shell
docker-compose up
```

To start both server after code changes (develop new things):

```shell
docker-compose up --build
```

To start single service:
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

Another approach to run single service, go to directory service then run `docker-compose up`.
```shell
cd service1
docker-compose up
```

## Start Server without Docker

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

## Test Application

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

### Concerns
Things to do or improve:
- Hot reload docker image to develop on local faster
- Multiple environment setup on docker-compose

Happy engineering!

---
