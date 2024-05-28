pub mod application;
pub mod config;
pub mod connections;
pub mod controller;
pub mod downstream;
pub mod migration;
pub mod repository;

// macro_rules! enhance_enum {
//     {$name: ident {
//         $($variant: ident,)+
//     }} => {
//         #[derive(Debug)]
//         pub enum $name {
//             $($variant,)+
//         }
//
//         impl $name {
//             pub fn name(&self) -> &'static str {
//                 match self {
//                     $(
//                         Self::$variant => stringify!($variant),
//                     )*
//                 }
//             }
//         }
//     }
// }
//
// enhance_enum! {
//     Color {
//         Red,
//         Blue,
//         Green,
//     }
// }
//
// type A = Color::Blue;

// macro_rules! id_type {
//     ($name: ident) => {
//         #[debug(Derive)]
//         struct $name(pub i32);

//         impl $name {
//             pub const MAX: Self = (i32::Self);

//             pub from_proto()
//         }
//     };
// }

// // #[macro_use]
// macro_rules! my_vec {
//     () => {
//         Vec::new()
//     };

//     ($($v: expr),+, $(,)?) => {
//         let mut vv = Vec::new();

//         $(
//             vv.push($v)
//         )+

//         vv
//     };

//     ($m: expr; $n: expr) => {{
//         let mut my_vec = Vec::new();

//         for _ in 0..$n {
//             my_vec.push($m)
//         }

//         my_vec
//    } }
// }

// fn meme() {
//     let a = my_vec![100; 6];
// }

// pub fn meme() {
//     let data = RefCell::new(5);

//     // immutable borrow
//     {
//         let borrowed = data.borrow();
//         println!("Current value: {}", *borrowed);
//     }

//     // mutable borrow
//     {
//         let mut borrowed_mut = data.borrow_mut();
//         *borrowed_mut += 1;
//     }

//     // immutable borrow
//     {
//         let borrowed = data.borrow();
//         println!("New value: {}", *borrowed);
//     }

//     struct MyStruct {
//         value: RefCell<i32>,
//     }

//     let a = MyStruct {
//         value: RefCell::new(30),
//     };

//     let mut b = a.value.borrow_mut();

//     // MyStruct {value: RefCell::new(100)};
//     *b = 100
// }
