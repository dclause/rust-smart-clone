use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
struct SimpleUnit;

// Will be expanded to :
// ```
// impl Clone for SimpleUnit {
//     fn clone (& self) -> Self {
//         Self {
//             *self
//         }
//     }
// }
// ```

fn main() {
    let unit = SimpleUnit;
    assert_eq!(unit.clone(), SimpleUnit);
}
