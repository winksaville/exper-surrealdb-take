# exper-take

Experiment with SurrealDB Response.take returned by a db.query call.

This is based on [this post](https://github.com/surrealdb/surrealdb/issues/2848#issuecomment-1765240638)
in issue [Feature: Add Implementations for QueryResult #2848](https://github.com/surrealdb/surrealdb/issues/2848).
It shows that you can deserialize into a Vec of structs but you have to name the
individual fields of the struct in the SurQL SELECT statement. Actually you can use '*'
instead of the file names.

The other thing I learned is that `take` allows you to
chain together [multiple query calls from a "db"](https://github.com/surrealdb/surrealdb/blob/2219388802dd9d769e17c15f6f3d94bb2cd64676/lib/src/api/method/query.rs#L280-L346)
the numeric parameter corresponds to the order of the queries but the take
can be out-of-order. ATM I don't have an example but will add one later. 

## Run

```bash
wink@3900x 24-05-17T23:34:57.954Z:~/prgs/SurrealDB/exper-take (main)
$ cargo run
   Compiling exper-take v0.1.0 (/home/wink/prgs/SurrealDB/exper-take)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.46s
     Running `target/debug/exper-take`
[src/main.rs:36:5] tobie = [
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "pss9mak2jn1tls9hy2rd",
            ),
        },
    },
]
[src/main.rs:47:5] tony = [
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "48xnbdpq7tc196zyu7jn",
            ),
        },
    },
]
[src/main.rs:55:5] people_take0 = [
    Person {
        name: "Tony Tiger",
        age: 50,
        is_active: false,
    },
    Person {
        name: "Tobie Hitchcock",
        age: 30,
        is_active: true,
    },
]
wink@3900x 24-05-17T23:39:16.512Z:~/prgs/SurrealDB/exper-take (main)cargo
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
