simple-rest-api
===

This is a VERY simple RESTful http server written in Rust. It serves one and one purpose only: to receive and store readings from AppleWatch.

NB this piece of code is largely a proof-of-concept and as such it MUST NOT be used in a production environment. Yeah, it's not been pentested yet, so don't you use it in production env!

## How to build it
First, get [rustup](https://rustup.rs) going on your system, and then select stable build as the default:

```sh
$ rustup default stable
```

Then, from within the main dir, simply run:

```sh
$ cargo build
```

## Configuring and running
After the binary was built successfully, you need to make sure you have the [diesel.rs](http://diesel.rs) CLI up
and running. But before you do this, first create an .env file with your path to your local Postgres DB:

```sh
$ echo DATABASE_URL=postgres://localhost/diesel_demo > .env
```

Next, install the CLI:

```sh
$ cargo install diesel_cli

```

Finally, run setup and redo the migrations:

```sh
$ diesel setup
$ diesel migration redo
```

After you have successfully run the above steps, you can start up the server by running:

```sh
$ ./target/debug/run <bind_address>
```

For example, to bind on your localhost address, run:

```sh
$ ./target/debug/run 127.0.0.1
```

And you're up receving the data via the POST request!

## Who should take the blame
Copyright (c) 2018 [Jakub Konka](http://www.jakubkonka.com)

