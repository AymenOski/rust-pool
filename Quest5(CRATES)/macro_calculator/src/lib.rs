use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_fats = 0.0;
    let mut total_proteins = 0.0;

    for food in foods {
        // Parse calories: "510kcal" -> 510.0
        // We replace "kcal" with nothing and parse the remaining string
        let cal_str = food.calories.1.replace("kcal", "");
        let cal_val: f64 = cal_str.parse().unwrap_or(0.0);

        // Accumulate values multiplied by portions
        total_cals += cal_val * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    // Round and create JSON object
    // We multiply by 100, round, then divide by 100 to get 2 decimal places
    object! {
        "cals": (total_cals * 100.0).round() / 100.0,
        "carbs": (total_carbs * 100.0).round() / 100.0,
        "proteins": (total_proteins * 100.0).round() / 100.0,
        "fats": (total_fats * 100.0).round() / 100.0,
    }
}