use std::collections::{HashMap, HashSet};

use mobs::*;

fn main() {
    // ------------------- Mob 1 -------------------
    let mut members_1 = HashMap::new();
    members_1.insert(
        "Alfred".to_owned(),
        Member {
            age: 50,
            role: Role::Soldier,
        },
    );
    members_1.insert(
        "Bruce".to_owned(),
        Member {
            age: 45,
            role: Role::Caporegime,
        },
    );

    let mut mob_1 = Mob {
        name: "Gotham Shadows".to_owned(),
        boss: Boss {
            age: 60,
            name: "Lucius Fox".to_owned(),
        },
        cities: HashSet::from(["Gotham".to_owned()]),
        wealth: 20000,
        members: members_1,
    };

    // ------------------- Mob 2 -------------------
    let mut members_2 = HashMap::new();
    members_2.insert(
        "Clark".to_owned(),
        Member {
            age: 35,
            role: Role::Soldier,
        },
    );
    members_2.insert(
        "Diana".to_owned(),
        Member {
            age: 40,
            role: Role::Associate,
        },
    );
    members_2.insert(
        "Barry".to_owned(),
        Member {
            age: 30,
            role: Role::Soldier,
        },
    );

    let mut mob_2 = Mob {
        name: "Metropolis Wolves".to_owned(),
        boss: Boss {
            age: 50,
            name: "Lex Luthor".to_owned(),
        },
        cities: HashSet::from(["Metropolis".to_owned()]),
        wealth: 15000,
        members: members_2,
    };

    // ------------------- Show initial state -------------------
    println!("--- Before Battle ---");
    println!("{:#?}", mob_1);
    println!("{:#?}", mob_2);

    // ------------------- Recruit & Promote -------------------
    mob_1.recruit(("Selina", 28));
    mob_1.recruit(("Harvey", 33));
    mob_2.recruit(("Kara", 25));

    mob_1.members.get_mut("Bruce").unwrap().get_promotion();
    mob_2.members.get_mut("Diana").unwrap().get_promotion();

    // ------------------- Battle -------------------
    mob_1.attack(&mut mob_2);

    // ------------------- Steal Wealth -------------------
    mob_2.steal(&mut mob_1, 5000);

    // ------------------- Conquer City -------------------
    mob_1.conquer_city(&[&mob_2], "Star City".to_owned());

    // ------------------- Show final state -------------------
    println!("--- After Battle ---");
    println!("{:#?}", mob_1);
    println!("{:#?}", mob_2);
}
