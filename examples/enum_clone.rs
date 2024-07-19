#![allow(unused_variables)]

use smart_clone::SmartClone;

#[derive(SmartClone, PartialEq, Debug, Default)]
enum SimpleEnum {
    #[default]
    A,
    B(usize, usize),
    C {
        x: u8,
        y: u8,
    },
    #[clone(SimpleEnum::D(8, 12))]
    D(i32, u32),
    #[clone(SimpleEnum::E { x: 3, y: 4 })]
    E {
        x: u8,
        y: u8,
    },
    #[clone(clone_with = "double")]
    F {
        x: u8,
        y: u8,
    },
    #[clone(default)]
    G {
        x: u8,
        y: u8,
    },
}

// Will be expanded to :
// ```
// impl Clone for SimpleEnum {
//     fn clone(&self) -> Self {
//         match self {
//             SimpleEnum::A => SimpleEnum::A,
//             SimpleEnum::B(v0, v1) => SimpleEnum::B(v0.clone(), v1.clone()),
//             SimpleEnum::C { x, y } => SimpleEnum::C {
//                 x: x.clone(),
//                 y: y.clone(),
//             },
//             SimpleEnum::D(..) => SimpleEnum::D(8, 12),
//             SimpleEnum::E { x, y } => SimpleEnum::E { x: 3, y: 4 },
//             SimpleEnum::F { x, y } => double(self),
//             SimpleEnum::G { x, y } => Default::default(),
//         }
//     }
// }
// ```

fn double(input: &SimpleEnum) -> SimpleEnum {
    match input {
        SimpleEnum::F { x, y } => SimpleEnum::F { x: x * 2, y: y * 2 },
        _ => unimplemented!(),
    }
}

fn main() {
    let enum_a = SimpleEnum::A;
    assert_eq!(enum_a.clone(), SimpleEnum::A);

    let enum_b = SimpleEnum::B(1, 2);
    assert_eq!(enum_b.clone(), SimpleEnum::B(1, 2));

    let enum_c = SimpleEnum::C { x: 1, y: 2 };
    assert_eq!(enum_c.clone(), SimpleEnum::C { x: 1, y: 2 });

    let enum_d = SimpleEnum::D(20, 30);
    assert_eq!(enum_d.clone(), SimpleEnum::D(8, 12));

    let enum_e = SimpleEnum::E { x: 1, y: 2 };
    assert_eq!(enum_e.clone(), SimpleEnum::E { x: 3, y: 4 });

    let enum_f = SimpleEnum::F { x: 2, y: 3 };
    assert_eq!(enum_f.clone(), SimpleEnum::F { x: 4, y: 6 });

    let enum_g = SimpleEnum::G { x: 7, y: 8 };
    assert_eq!(enum_g.clone(), SimpleEnum::A);
}
