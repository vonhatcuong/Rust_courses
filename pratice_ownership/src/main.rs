// fn vec_diff(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
//     let mut rv = Vec::new();
//     for (l, r) in left.iter().zip(right.iter()) {
//         rv.push(l - r);
//     }
//     return rv;
// }

// /// Returns true if and only if all elements in `vec` are equal to `value`.
// fn all(vec: Vec<i32>, value: i32) -> bool {
//     for elem in vec.iter() {
//         if *elem != value {
//             return false;
//         }
//     }
//     return true;
// }

// fn main() {
//     let v1 = vec![0, 1, 2];
//     let v2 = vec![3, 4, 5];
//     let diff = vec_diff(&v2, &v1);

//     let diff2 = vec_diff(&diff, &v1);
//     println!("{:?}", all(diff, 3));
// }

// fn main() {
//     let objetive = 3126.59;

//     // 27
//     let values: Vec<f64> = vec![
//         2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
//         139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
//         4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
//     ];

//     let values_number = values.len();
//     let values_index_max = values_number - 1;

//     let mut additions: Vec<usize> = vec![0];

//     println!("{:?}", values_number);

//     while additions.len() > 0 {
//         let mut addition: f64 = 0.0;
//         let mut saltar: i32 = 0;

//         // Sumar valores en additions
//         for element_index in &additions {
//             let addition_aux = values[*element_index];
//             addition = addition_aux + addition;
//         }
//     }
// }

// fn main() {}

// pub fn iter_num(num: i32) -> bool {
//     let num_str = num.to_string();
//     let chars = num_str.chars(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
//     let len = chars.clone().count(); // <-- `chars` moved due to this method call

//     println!("Len = {:?}", len);

//     for c in chars {
//         // <-- ❌ "value used here after move": chars
//         println!(">>> {:?}", c);
//     }

//     return true;
// }

fn main() {
    let mut v = vec![1, 2, 3];

    go(&mut v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

fn go(v: &mut Vec<i32>) {
    for i in v.into_iter() {
        println!("{}", i);
    }
    v.push(4);
}
