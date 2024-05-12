// fn abc () -> () {
//     #[derive(PartialEq)]
//     struct MyType {
//         value: i32,
//     }

//     #[derive(PartialEq)]
//     struct OtherType {
//         other_value: i32,
//     }

//     impl PartialEq<OtherType> for MyType {
//         fn eq(&self, other: &OtherType) -> bool {
//             self.value == other.other_value
//         }
//     }

//     impl PartialEq<MyType> for OtherType {
//         fn eq(&self, other: &MyType) -> bool {
//             self.other_value == other.value
//         }
//     }

//     let a = MyType{ value: 1i32 };
//     let b = OtherType{ other_value: 2i32 };

//     //  this equality is made possible by PartialEq
//     if a == b {

//     }

//     impl Eq for OtherType {}
//     impl Eq for MyType {}

// }
