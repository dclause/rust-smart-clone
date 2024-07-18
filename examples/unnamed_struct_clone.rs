use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
struct Point3D(
    #[clone(default)]
    i32,
    #[clone(vec ! [2])]
    vec![],
    #[clone(clone_with = "SimpleStruct::vec_clone")]
    vec![1, 2, 3],
);

impl Point3D {
    fn vec_clone(input: &Vec<u32>) -> Vec<u32> {
        input.iter().map(|i| i * 2).collect()
    }
}

// Will be expanded to :
// ```
// impl Clone for Point3D {
//     fn clone (& self) -> Self {
//         Point3D(self.0, self.1, self.2)
//     }
// }
// ```

fn main() {
    let point = Point3D(1, 2, 3);
    ;
    assert_eq!(point.clone(), Point3D(1, 2, 3));
}