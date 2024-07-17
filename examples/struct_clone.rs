use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
struct Foo {
    #[clone = 12]
    a: i32,
    b: i32,
    #[clone(Some(Default::default()))]
    c: Option<i32>,
    #[clone(fn = Foo::vec_clone)]
    d: Vec<u32>,
    #[clone("banana".to_owned())]
    e: String,
}

impl Foo {
    fn vec_clone(input: Vec<u32>) -> Vec<u32> {
        input.iter().map(|i| i + 1).collect()
    }
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Self {
            a: 12,
            b: self.b.clone(),
            c: Some(Default::default()),
            d: Foo::vec_clone(self.d.clone()),
            e: "banana".to_owned(),
        }
    }
}

fn main() {
    let foo = Foo {
        a: 42,
        b: 8,
        c: Some(42),
        d: vec![1, 2, 3],
        e: "apple".to_owned(),
    };
    assert_eq!(foo.clone(), Foo {
        a: 12,
        b: 8,
        c: Some(0),
        d: vec![4, 5, 6],
        e: "banana".to_owned(),
    });
}