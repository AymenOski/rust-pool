//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    (km_h * 1000.0) / (60.0 * 60.0)
}

/*
    *expression vs statement
        Every let name = <anything>; needs that semicolon at the end.
        so when we have something that ends with a semicolon, it is a statement. It does not return a value.
        when we have something that does not end with a semicolon, it is an expression. It returns a value.
        for example `5 + 5` is an expression that returns a value, while `let x = 5 + 5;` is a statement that does not return a value.
*/
