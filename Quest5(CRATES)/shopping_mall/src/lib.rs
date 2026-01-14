pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max_sq_meters: u64 = 0;
    let mut name_store = String::new();
    let mut name_floor = String::new();
    for (name_fl, floor) in &mall.floors {
        for (name_st, info) in floor.stores.iter() {
            if max_sq_meters < info.square_meters {
                max_sq_meters = info.square_meters;
                name_store = name_st.clone();
                name_floor = name_fl.clone();
            }
        }
    }
    let store = mall
        .floors
        .get(&name_floor)
        .unwrap()
        .stores
        .get(&name_store)
        .unwrap();
    (name_store, store.clone())
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut sallry: f64 = 0.0;

    let mut res: Vec<(&String, &Employee)> = vec![];

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name_emp, emp) in store.employees.iter() {
                if sallry < emp.salary {
                    sallry = emp.salary;
                    res.clear();
                    res.push((name_emp, emp));
                } else if sallry == emp.salary {
                    res.push((name_emp, emp));
                }
            }
        }
    }

    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut nb_person = 0;

    for floor in mall.floors.values() {

        for store in floor.stores.values() {
            nb_person += store.employees.len();
        }
    }

    nb_person += mall.guards.len();

    nb_person
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut n_guard = mall.guards.len();
    let mut total_area: u64 = 0;
    for (_, store) in &mall.floors {
        total_area += store.size_limit;
    }
    let number_quards: u64 = total_area / 200;

    for (guard_name, info) in guards {
        mall.hire_guard(guard_name.clone(), info);
        n_guard += 1;
        if n_guard == number_quards.try_into().unwrap() {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for info in floor.stores.values_mut() {
            for emp in info.employees.values_mut() {
                let working_hours = emp.working_hours.1 - emp.working_hours.0;
                let ten_percente = emp.salary / 10.0;

                if working_hours >= 10 {
                    emp.raise(ten_percente);
                } else {
                    emp.cut(ten_percente);
                }
            }
        }
    }
}