use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
struct SimpleStruct {
    a: u8, // left for standard clone
    b: (i32, u32), // left for standard clone
    c: Option<i32>, // left for standard clone
    d: Vec<u32>, // left for standard clone
    #[clone = 12]
    e: u8, // Override with hardcoded value (same as `#[clone(12)]`)
    #[clone(TEST)]
    f: u8, // In general, prefer this syntax for hardcode, variable or const
    #[clone((42, 69))]
    g: (i32, u32),
    #[clone(default)]
    h: Vec<Vec<Vec<(u8, u8)>>>, // Reserved 'skip' keyword to clone to Default::default() value (g type must implement `Default`)
    #[clone(Some(Default::default()))] // `Some(Default::default())` is not `None` but `Some(0)` !
    i: Option<i32>,
    #[clone(clone_with = "SimpleStruct::vec_clone")]
    j: Vec<u32>,
    #[clone("banana".to_owned())]
    k: String,
}

const TEST: u8 = 3;

// This will be expanded for you:
// ```
// impl Clone for SimpleStruct {
//     fn clone(&self) -> Self {
//         Self {
//             a: self.a.clone(),
//             b: self.b.clone(),
//             c: self.c.clone(),
//             d: self.d.clone(),
//             e: 12,
//             f: TEST,
//             g: (42, 69),
//             h: Default::default(),
//             i: Some(Default::default()),
//             j: SimpleStruct::vec_clone(&self.j),
//             k: "banana".to_owned(),
//         }
//     }
// }
// ```


impl SimpleStruct {
    fn vec_clone(input: &Vec<u32>) -> Vec<u32> {
        input.iter().map(|i| i * 2).collect()
    }
}

fn main() {
    let foo = SimpleStruct {
        a: 42,
        b: (8, 8),
        c: Some(42),
        d: vec![1, 2, 3],
        e: 0,
        f: 0,
        g: (0, 0),
        h: vec![vec![vec![(1, 2)]]],
        i: Some(24),
        j: vec![1, 2, 3],
        k: "apple".to_string(),
    };
    assert_eq!(foo.clone(), SimpleStruct {
        a: 42,
        b: (8, 8),
        c: Some(42),
        d: vec![1, 2, 3],
        e: 12,
        f: 3,
        g: (42, 69),
        h: vec![],
        i: Some(0),
        j: vec![2, 4, 6],
        k: String::from("banana"),
    });
}