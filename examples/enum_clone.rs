use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug)]
enum CustomCloneEnum {
    A,
    #[clone = Some(42)]
    B,
    #[clone(custom_clone_expr)]
    C,
    #[clone(fn = custom)]
    D,
}

fn main() {
    let enum_a = CustomCloneEnum::A;
    assert_eq!(enum_a.clone(), CustomCloneEnum::A);
}