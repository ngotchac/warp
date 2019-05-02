#![deny(warnings)]
extern crate futures;
extern crate pretty_env_logger;
extern crate warp;

use std::path::PathBuf;

use warp::{Filter};

fn main() {
    pretty_env_logger::init();

    let readme = warp::get2()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    // dir already requires GET...
    let examples = warp::path("ex")
        .and(warp::path::tail())
        .and(warp::fs::conditionals())
        .and_then(|filename: warp::path::Tail, conditionals: warp::fs::Conditionals| {
            let filepath = PathBuf::from("./examples").join(filename.as_str());
            println!("Serving {:?}", filepath);
            warp::fs::send_file(filepath, conditionals)
        });

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
