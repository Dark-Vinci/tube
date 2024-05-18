pub mod application;
pub mod config;
pub mod connections;
pub mod controller;
pub mod downstream;
pub mod migration;
pub mod repository;

// macro_rules! id_type {
//     ($name: ident) => {
//         #[debug(Derive)]
//         struct $name(pub i32);
//
//         impl $name {
//             pub const MAX: Self = (i32::Self);
//
//             pub from_proto()
//         }
//     };
// }
//
// // #[macro_use]
// macro_rules! my_vec {
//     () => {
//         Vec::new()
//     };
//
//     ($($v: expr),+, $(,)?) => {
//         let mut vv = Vec::new();
//
//         $(
//             vv.push($v)
//         )+;
//
//         vv
//     };
//
//     ($m: expr; $n: expr) => {{
//         let mut my_vec = Vec::new();
//
//         for _ in 0..$n {
//             my_vec.push($m)
//         }
//
//         my_vec
//    } }
// }
//
// fn meme() {
//     let a = my_vec![100; 6];
// }
