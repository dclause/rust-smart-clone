use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
struct Point4D(
    i32,
    #[clone(default)]
    i32,
    #[clone(String::from("banana"))]
    String,
    #[clone(clone_with = "Point4D::vec_clone")]
    Vec<u32>,
);

impl Point4D {
    fn vec_clone(input: &Vec<u32>) -> Vec<u32> {
        input.iter().map(|i| i * 2).collect()
    }
}

// Will be expanded to :
// ```
// impl Clone for Point4D {
//     fn clone(&self) -> Self {
//         Point4D {
//             0: self.0.clone(),
//             1: Default::default(),
//             2: String::from("banana"),
//             3: Point4D::vec_clone(&self.3),
//         }
//     }
// }
// ```

fn main() {
    let point = Point4D(1, 2, String::from("apple"), vec![1, 2, 3]);
    assert_eq!(point.clone(), Point4D(1, 0, "banana".into(), vec![2, 4, 6]));
}