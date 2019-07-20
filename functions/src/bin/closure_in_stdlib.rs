fn main() {
    //
    // ANY
    //
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // .iter() for vec yields &i32. Destructure to i32
    println!("2 in v1: {}", v1.iter().any(|&x| x == 2));
    // .into_iter() for vec yields i32
    println!("2 in v2: {}", v2.into_iter().any(|x| x == 2));

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    // .iter() for array yields &i32. Destructure to i32
    println!("2 in a1: {}", a1.iter().any(|&x| x == 2));
    // .into_iter() for array unusualy yields &i32. Destructure to i32
    println!("2 in a2: {}", a2.into_iter().any(|&x| x == 2));

    //
    // FIND
    //
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // iter() yields &i32. Find accept reference to &32 so &&i32 desctructured into i32
    println!("Find 2 in v1: {:?}", v1.iter().find(|&&x| x == 2));
    // iter() yields i32. Find accept reference to 32 so &i32 desctructured into i32
    println!("Find 2 in v2: {:?}", v2.into_iter().find(|&x| x == 2));

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    println!("Find 2 in a1: {:?}", a1.iter().find(|&&x| x == 2));
    println!("Find 2 in a2: {:?}", a2.iter().find(|&&x| x == 2));
}