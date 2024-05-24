// use std::cell::Cell;
// use std::cmp::Ordering;

// // #[derive(PartialEq, PartialOrd)]
// // struct MyType {
// //     v1: i32,
// //     v2: f64,
// // }

// #[derive(PartialEq, PartialOrd)]
// struct OtherType {
//     val1: i32,
//     val2: f64,
// }

// impl PartialEq<OtherType> for MyType {
//     fn eq(&self, other: &OtherType) -> bool {
//         self.v1 == other.val1
//     }
// }

// impl PartialOrd<OtherType> for MyType {
//     // required method
//     fn partial_cmp(&self, other: &OtherType) -> Option<Ordering> {
//         return self.v1.partial_cmp(&other.val1)
//     }
//     // not required
//     fn lt(&self, other: &OtherType) -> bool {
//         return self.v1 < other.val1;
//     }
//     // not required
//     fn gt(&self, other: &OtherType) -> bool {
//         return self.v1 > other.val1;
//     }
//     // not required
//     fn le(&self, other: &OtherType) -> bool {
//         return self.v1 <= other.val1;
//     }
//      // not required
//     fn ge(&self, other: &OtherType) -> bool {
//         return self.v1 >= other.val1;
//     }
// }

// pub fn useage() {
//     let a = MyType{v1: 20, v2: 20.0};
//     let b = OtherType{val1: 30, val2: 30.0};

//     if a > b {
//         println!("MyType is Greater than OtherType")
//     }
// }

// // // case 1: type id
// // use std::any::{Any, TypeId};

// // fn is_string(s: &dyn Any) -> bool {
// //     TypeId::of::<String>() == s.type_id()
// // }

// // // case 2: downcast
// // fn print_if_string(value: Box<dyn Any>) {
// //     if let Ok(myType) = value.downcast::<MyType>() {
// //         println!("MyType ({}): {}", myType.v1 myType.v2);
// //     }
// // }

// // // case 3: type checking
// // fn is_string(s: &dyn Any) {
// //     if s.is::<OtherType>() {
// //         println!("It's an othertype!");
// //     } else {
// //         println!("Not an othertype");
// //     }
// // }

// use std::marker::PhantomData;
// use std::sync::{Arc, Barrier};
// use std::{mem, thread};
// use anyhow::Ok;
// use rand::Rng;
// use std::time::Duration;

// fn main() {
//     let num_threads = 100;

//     let barrier = Arc::new(Barrier::new(num_threads));

//     // Create threads
//     let mut threads = Vec::with_capacity(num_threads);

//     for i in 0..=num_threads {
//         let barrier = Arc::clone(&barrier);

//         let thread = thread::spawn(move || {
//             // generate random number between 0 to 5
//             let mut rng = rand::thread_rng();
//             let sleep_duration = Duration::from_secs(rng.gen_range(1..=5));

//             // sleep for the amount of randomly generated time
//             thread::sleep(sleep_duration);

//             // Wait for all threads to reach the barrier
//             println!("Thread {} aiming", i);
//             barrier.wait();

//             println!("Thread {} just Fired", i);
//         });
//         threads.push(thread);
//     }

//     // wait for all threads
//     for thread in threads {
//         thread.join().unwrap();
//     }

//     println!("All threads finished execution");
// }

// use std::ops::Deref;

// #[derive(PartialEq, PartialOrd)]
// struct MyType {
//     v1: i32,
//     v2: f64,
// }

// struct AType(MyType);

// impl Deref for AType {
//     type Target = i32;

//     fn deref(&self) -> &Self::Target {
//         return &self.0.v1;
//     }
// }

// pub fn func () {
//     use std::ops::Deref;

//     #[derive(PartialEq, PartialOrd)]
//     struct MyType {
//         v1: i32,
//         v2: f64,
//     }

//     struct AType(MyType);

//     impl Deref for AType {
//         type Target = i32;

//         fn deref(&self) -> &Self::Target {
//             return &self.0.v1;
//         }
//     }

//     let a = MyType{v1: 20, v2: 10.0};
//     let b = AType(a);

//     assert!(*b == 20);

//     // case 1
//     impl Drop for AType {
//         fn drop(&mut self) {
//             print!("Cleanup work goes in here");
//             println!("Looks like defer function to me");
//         }
//     }

//     // case 2
//     pub struct Defer<F: FnOnce()>(Option<F>);

//     impl<F: FnOnce()> Defer<F> {
//         pub fn new(f: F) -> Self {
//             Defer(Some(f))
//         }
//     }

//     impl<F: FnOnce()> Drop for Defer<F> {
//         fn drop(&mut self) {
//             if let Some(f) = self.0.take() {
//                 f();
//             }
//         }
//     }

//     fn main() {
//         let _ = Defer::new(|| {
//             println!("Deferred code executed");
//         });

//         println!("Main code");

//         // Deferred code will be executed when `defer` goes out of scope
//     }
// }

// struct MyStruct {
//     value: u32,
// }

// impl AsRef<u32> for MyStruct {
//     fn as_ref(&self) -> &u32 {
//         &self.value
//     }
// }

// impl AsMut<u32> for MyStruct {
//     fn as_mut(&mut self) -> &mut u32 {
//         &mut self.value
//     }
// }
// #[feature(cell_update)]

// pub fn mainjjj() {
//     struct Custom {
//         name: String,
//         field: Cell<u8>,
//     }

//     // note the this is not a mutable value
//     let custom = Custom {
//         name: "Custom".into(),
//         field: Cell::new(1),
//     };

//     // value is now 100
//     custom.field.set(100);

//     let c2 = Cell::new(10u8);

//     // value is now 10
//     custom.field.swap(&c2);

//     // trait MyTrait {}

//     // case 1
//     struct Cloud {
//         color: String,
//         r#type: CloudType,
//     }

//     impl Default for Cloud {
//         fn default() -> Self {
//             Self {
//                 color: "Black".into(),
//                 r#type: Default::default(),
//             }
//         }
//     }

//     let _cloud: Cloud = Default::default();

//     // case 2
//     #[derive(Default)]
//     enum CloudType {
//         #[default]
//         Stormy,
//         Rainy,
//         Others,
//     }

//     // let cloud: Cloud = Default::default();

//     // case 1
//     struct MySlice<'a, T> {
//         range: *const T,
//         phantom: PhantomData<&'a T>,
//     }

//     // case 2
//     trait MyTrait {}

//     struct External<T> {
//         resource_handle: *const f64,
//         resource_type: PhantomData<T>,
//     }

//     impl<T: MyTrait> External<T> {
//         fn new() -> Self {
//             Self {
//                 resource_handle: &32.0 as *const f64,
//                 resource_type: PhantomData,
//             }
//         }
//     }

//     struct Human {
//         age: u8,
//     }

//     struct Adult {
//         age: u8,
//     }

//     impl TryInto<u8> for Adult {
//         type Error = String;

//         fn try_into(self) -> Result<u8, Self::Error> {
//             if self.age % 2 != 0 {
//                 return Err("With an ODD age".into())
//             }

//             Ok(self.age)
//         }
//     }

//     impl TryFrom<Human> for Adult {
//         type Error = String;

//         fn try_from(h: Human) -> Result<Self, Self::Error> {
//             if h.age < 18 {
//                 return Err("not yet an adult".into())
//             }

//             Ok(Self {
//                 age: h.age,
//             })
//         }
//     }

//     let human = Human{age: 32};

//     let adult: Result<Adult, _> = human.try_into();

//     let _odd_age: Result<u8, _> = adult.unwrap().try_into();

//     // println!("{adult}");

//     struct MyStruct {
//         value: u32,
//     }

//     impl From<f64> for MyStruct {
//         fn from(value: f64) -> Self {
//             Self {
//                 value: value as u32,
//             }
//         }
//     }

//     impl Into<f64> for MyStruct {
//         fn into(self) -> f64 {
//             self.value as f64
//         }
//     }

//     // enabled by from trait
//     let my_struct: MyStruct = 23.0.into();
//     // enabled by into trait
//     let value: f64 = my_struct.into();

//     println!("{value}");

//     // my_struct.v

//     // as ref trait
//     impl AsRef<u32> for MyStruct {
//         fn as_ref(&self) -> &u32 {
//             &self.value
//         }
//     }

//     // as mut trait
//     impl AsMut<u32> for MyStruct {
//         fn as_mut(&mut self) -> &mut u32 {
//             &mut self.value
//         }
//     }

//     let mut my_struct = MyStruct { value: 42 };

//     let value_ref: &u32 = my_struct.as_ref();

//     println!("AsRef Value: {}", value_ref);

//     *my_struct.as_mut() += 10

//     // Using AsMut
//     // *my_struct.as_mut() += 10;
//     // println!("Updated value: {}", my_struct.value);

// }

//
// use std::{marker::PhantomData, ops::Add};
//
// #[macro_use]
// mod a {
//     macro_rules! assert_eq_len {
//         ($x: expr, $y: expr, $func: ident, $op:tt) => {
//             assert!(
//                 $x.len() == $y.len(),
//                 "{:?} DIMENSION MISMATCH {:?}| {:?}| {:?}",
//                 stringify!($func),
//                 ($x.len()),
//                 stringify!($op),
//                 ($y.len()),
//             );
//         };
//     }
//
//     macro_rules! op {
//         ($func: ident, $bound: ident, $op: tt, $method: ident) => {
//             fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
//                 assert_eq_len!(xs, ys, $func, $op);
//
//                 for (x, y) in xs.iter_mut().zip(ys.iter()) {
//                     *x = $bound::$method(*x, *y);
//                 }
//             }
//         };
//     }
// }
//
// op!(add_assign, Add, +=, add);
