fn main() {
    let mut array: [i32; 3] = [0, 1, 2];
    //A repeat expression [x; N], which produces an array with N copies of x.

    array[0] = 10;

    for x in 0..10 {
        print!("{x} ");
    }

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // Insert `elem` in the vector.
    vec.push(1i8);

    let arr = [0, 1, 2, 3, 4];
    assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
    assert_eq!(arr[ .. 3], [0, 1, 2      ]);
    assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
    assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
    assert_eq!(arr[1.. 3], [   1, 2      ]); // This is a `Range`
    assert_eq!(arr[1..=3], [   1, 2, 3   ]);
}