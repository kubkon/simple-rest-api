simple-rest-api
===

This is a VERY simple RESTful http server written in Rust. It serves one and one purpose only: to receive and store readings from AppleWatch.

NB this piece of code is largely a proof-of-concept and as such it MUST NOT be used in a production environment. Yeah, it's not been pentested yet, so don't you use it in production env!

## How to build it
First, get [rustup](https://rustup.rs) going on your system, and then select nightly build as the default:

```sh
$ rustup default nightly
```

Then, from within the main dir, simply run:

```sh
$ cargo run
```

And you're up receving the data via the POST request!

## Who should take the blame
Copyright (c) 2018 [Jakub Konka](http://www.jakubkonka.com)

