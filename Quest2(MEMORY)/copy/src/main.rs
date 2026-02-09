use copy::*;

fn main() {
    let a = "1 2 4 5 6".to_owned();
    let b = vec![1, 2, 4, 5];
    let c = 0;

    println!("{:?}", nbr_function(c));
    println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a));
    // if we try to print a here after calling str_function(a), we will get an error because a is moved into the function and we cannot use it after that.
    // println!("{}", a); // this will cause an error because a is moved into the function
}

// $ cargo run
// (0, 1.0, -inf)
// ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003])
// ("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")
// $
