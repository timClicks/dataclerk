/// build.rs
///
/// The intention here is to use build.rs to include a static lib of sqlite3.
/// Am currently finding the bindgen errors more irritating to work with than I would,
/// like, so I'm parking this work until I get a proof of concept up
///
/// Much of the material here is derived from https://github.com/jgallagher/rusqlite/blob/master/libsqlite3-sys/build.rs

// extern crate bindgen;
// extern crate cc;

// use bindgen::callbacks::{IntKind, ParseCallbacks};

// #[derive(Debug)]
// struct SqliteTypeChooser;

// impl ParseCallbacks for SqliteTypeChooser {
//     fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
//         if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
//             Some(IntKind::I32)
//         } else {
//             None
//         }
//     }
// }

fn main() {
    // let bindings =  bindgen::builder()
    //     .header("src/sqlite3.h")
    //     .header("src/sqlite3ext.h")
    //     .parse_callbacks(Box::new(SqliteTypeChooser))
    //     .rustfmt_bindings(true);
    // let bindings = bindings.clang_arg("-DSQLITE_ENABLE_SESSION");
    // bindings
    //     .generate()
    //     .expect("unable to generate sqlite3 bindings");

    // cc::Build::new()
    //     .file("src/sqlite3.c")
    //     .define("SQLITE_CORE", None)
    //     .define("HAVE_USLEEP", Some("1"))
    //     .define("SQLITE_DEFAULT_FOREIGN_KEYS", Some("1"))
    //     .define("SQLITE_ENABLE_API_ARMOR", None)
    //     .define("SQLITE_ENABLE_COLUMN_METADATA", None)
    //     .define("SQLITE_ENABLE_DBSTAT_VTAB", None)
    //     .define("SQLITE_ENABLE_FTS3_PARENTHESIS", None)
    //     .define("SQLITE_ENABLE_FTS3", None)
    //     .define("SQLITE_ENABLE_FTS5", None)
    //     .define("SQLITE_ENABLE_JSON1", None)
    //     .define("SQLITE_ENABLE_LOAD_EXTENSION", Some("1"))
    //     .define("SQLITE_ENABLE_MEMORY_MANAGEMENT", None)
    //     .define("SQLITE_ENABLE_RTREE", None)
    //     .define("SQLITE_ENABLE_SESSION", None)
    //     .define("SQLITE_ENABLE_STAT2", None)
    //     .define("SQLITE_ENABLE_STAT4", None)
    //     .define("SQLITE_HAVE_ISNAN", None)
    //     .define("SQLITE_SOUNDEX", None)
    //     .define("SQLITE_THREADSAFE", Some("1"))
    //     .define("SQLITE_USE_URI", None)
    //     .compile("sqlite");
}
