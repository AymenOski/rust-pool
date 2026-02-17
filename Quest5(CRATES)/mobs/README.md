# Mobs: Mastering Rust's Module System

A hands-on project to learn **Packages, Crates, and Modules** in Rust through building an organized crime simulation library.

---

## 📚 What You'll Learn

### Core Concepts

| Concept | What It Is | Why It Matters |
|---------|-----------|----------------|
| **Packages** | Container for one/multiple crates | Organize related projects together |
| **Crates** | Tree of modules (binary or library) | The compilation unit in Rust |
| **Modules** | Organize code into namespaces | Control scope and privacy |
| **Use Statement** | Import items into scope | Access code with shorter paths |
| **Privacy** | `pub` keyword controls visibility | Hide implementation details |
| **Paths** | Navigate the module tree | Find and reference items |

### Skills You'll Gain

✅ **File Organization** - Split code across multiple files logically  
✅ **Module Hierarchy** - Create nested module structures  
✅ **Re-exports** - Use `pub use` to expose internal code  
✅ **Privacy Control** - Make items `pub` or private intentionally  
✅ **Encapsulation** - Hide complexity behind clean APIs  
✅ **Scope Management** - Understand name resolution in nested contexts

---

## 🏗️ Project Architecture

### File Structure

```
mobs/
├── Cargo.toml              # Package manifest
├── src/
│   ├── lib.rs             # Library root (declares modules)
│   ├── main.rs            # Binary entry point (optional)
│   ├── mobs.rs            # Main Mob struct
│   └── mobs/              # Submodules folder
│       ├── boss.rs        # Boss struct module
│       └── member.rs      # Member & Role enum module
└── README.md              # This file
```

### Module Hierarchy (Tree Structure)

```
crate
├── mobs (mod mobs in lib.rs)
│   ├── Mob (struct)
│   ├── boss (submodule)
│   │   └── Boss (struct)
│   └── member (submodule)
│       ├── Role (enum)
│       └── Member (struct)
└── [re-exported via pub use]
```

---

## 🔑 Key Concepts Explained

### 1. **Packages vs Crates**

**Package** = Project folder with `Cargo.toml`  
**Crate** = The compiled Rust code (binary or library)

```
One package can have:
- ONE library crate (lib.rs)
- MANY binary crates (src/bin/*)
```

**Good Code:**
```rust
// Cargo.toml - clearly defines what we're building
[package]
name = "mobs"          # Package name
version = "0.1.0"

# One library crate
# Multiple binaries could go in src/bin/
```

**Problem Code:**
```rust
// ❌ Having unclear structure makes it confusing
// Multiple unrelated crates in same folder
```

---

### 2. **Modules & Organization**

**Why Modules?**
- Group related functionality
- Control visibility (public/private)
- Create namespaces to avoid name conflicts

**Example: Module Declaration**

[lib.rs](src/lib.rs) - Entry point declares submodules:

```rust
mod mobs;           // Declares mobs module
pub use mobs::*;    // Re-exports everything from mobs

// Now users can do:
// use mobs::{Mob, Boss, Role};
```

[mobs.rs](src/mobs.rs) - Main module declares submodules:

```rust
pub mod boss;      // Declares boss submodule
pub mod member;    // Declares member submodule

pub struct Mob {
    name: String,
    boss: Boss,
    // ... other fields
}
```

**File vs Module Naming:**

| File Path | Module Path | Usage |
|-----------|------------|-------|
| `src/mobs.rs` | `mod mobs` | Declare in parent module |
| `src/mobs/boss.rs` | `mod boss` inside mobs | Submodule in folder |
| `src/mobs/member.rs` | `mod member` inside mobs | Submodule in folder |

---

### 3. **Privacy & Visibility**

**Rule: Everything is private by default**

```rust
// ❌ Private - can't access from outside module
struct Boss {
    name: String,
}

// ✅ Public - accessible outside module
pub struct Boss {
    name: String,
}

// ✅ Public method on private struct
impl Boss {
    pub fn new(name: &str) -> Boss { /* ... */ }
}

// ❌ Private method - restricted
impl Boss {
    fn secret_method(&self) { /* ... */ }
}
```

**Access Rules:**

```
Parent module ----can access----> Child module (private items)
Child module ----CANNOT access----> Parent module (private items)
Sibling modules ----CANNOT access----> Each other (private items)

Only PUBLIC (pub) items are accessible across boundaries!
```

---

### 4. **Paths: Finding Items in the Module Tree**

**Absolute Path** - Start from crate root:
```rust
use crate::mobs::member::Role;
use crate::mobs::Boss;
```

**Relative Path** - Start from current module:
```rust
// Inside mobs.rs looking for member module:
use self::member::Role;
use member::Member;
```

**Using Use Statements:**
```rust
// Bring items into scope
use mobs::{Mob, Boss};
use mobs::member::Role;

// Then use directly instead of full path
let role = Role::Associate;  // Instead of mobs::member::Role::Associate
```

---

## 💾 Data Flow Example

### Creation & Modification Flow

```
User Code
   │
   ├──> Create Mob ────> Mob::new()
   │                       │
   │                       └──> Boss::new() [from boss module]
   │
   ├──> recruit(member) ──> Insert into HashMap<String, Member>
   │
   ├──> attack(other_mob) ─> Calculate power scores
   │                         ├──> Sum Role values (member module)
   │                         └──> Remove weakest members
   │
   ├──> steal(target, amount) ─> Transfer wealth
   │
   └──> conquer_city(mobs, city) ─> Add to HashSet if not owned
```

### Power Score Calculation

```
Role → Score
────────────
Underboss    → 4
Caporegime   → 3
Soldier      → 2
Associate    → 1

Example: Mob has 2 Soldiers, 1 Caporegime, 3 Associates
Power = (2×2) + (1×3) + (3×1) = 10 points
```

---

## 📋 Implementation Guide

### Step 1: Set Up Module Files

Create the module structure:

```bash
touch src/mobs/boss.rs
touch src/mobs/member.rs
```

### Step 2: Boss Module [src/mobs/boss.rs]

```rust
#[derive(Debug, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u32,
}

impl Boss {
    /// Create a new boss
    /// 
    /// # Arguments
    /// * `name` - Boss's name
    /// * `age` - Boss's age
    ///
    /// # Example
    /// ```
    /// let boss = Boss::new("Vito", 50);
    /// ```
    pub fn new(name: &str, age: u32) -> Boss {
        Boss {
            name: name.to_string(),
            age,
        }
    }
}
```

**Line-by-line breakdown:**
- `#[derive(Debug, PartialEq)]` - Automatically implement these traits
- `pub struct Boss` - Struct is public (users can create it)
- `pub fn new()` - Associated function (not a method, no `self`)
- `name.to_string()` - Convert `&str` to owned `String`
- `return Boss { ... }` - Implicit return (no semicolon)

### Step 3: Member Module [src/mobs/member.rs]

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Associate = 1,
    Soldier = 2,
    Caporegime = 3,
    Underboss = 4,
}

impl Role {
    /// Get the combat power value of this role
    pub fn power(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn new(role: Role, age: u32) -> Member {
        Member { role, age }
    }

    /// Promote member to next rank
    /// Panics if already Underboss
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => panic!("Already at maximum rank!"),
        };
    }
}
```

**Key points:**
- `#[derive(Clone, Copy)]` on enum allows member reassignment
- `= 1, = 2` assigns integer values for power calculation
- `*self as u64` dereferences and casts to u64
- `panic!()` stops execution (fatal error) for impossible cases

### Step 4: Main Mob Struct [src/mobs.rs]

```rust
use std::collections::{HashMap, HashSet};

pub mod boss;
pub mod member;

pub use boss::Boss;
pub use member::{Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn new(name: &str, boss: Boss) -> Mob {
        Mob {
            name: name.to_string(),
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    /// Add a new member to the mob
    /// Role is automatically set to Associate
    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        let member = Member::new(Role::Associate, age);
        self.members.insert(name.to_string(), member);
    }

    /// Calculate total combat power score
    fn calculate_power(&self) -> u64 {
        self.members.values().map(|m| m.role.power()).sum()
    }

    /// Attack another mob, removing weakest members
    pub fn attack(&mut self, other: &mut Mob) {
        let my_power = self.calculate_power();
        let their_power = other.calculate_power();

        let (loser, winner) = if my_power > their_power {
            (other, self)
        } else if their_power > my_power {
            (self, other)
        } else {
            // Draw: attacker loses
            (self, other)
        };

        // Remove youngest member from loser
        if let Some((name, _)) = loser.members
            .iter()
            .min_by_key(|(_, m)| m.age)
            .map(|(n, m)| (n.clone(), m.clone()))
        {
            loser.members.remove(&name);
        }

        // If loser is eliminated, winner takes everything
        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    /// Steal wealth from target mob
    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen = amount.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    /// Conquer a city if no other mob owns it
    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city: String) {
        let city_owned = other_mobs.iter()
            .any(|mob| mob.cities.contains(&city));

        if !city_owned {
            self.cities.insert(city);
        }
    }
}
```

**Method breakdown:**

| Method | Input | Output | Effect |
|--------|-------|--------|--------|
| `recruit()` | `(&str, u32)` | `()` | Insert member to HashMap |
| `calculate_power()` | `()` | `u64` | Sum all member role values |
| `attack()` | `&mut Mob` | `()` | Modify both mobs (fight!) |
| `steal()` | `&mut Mob, u64` | `()` | Transfer wealth |
| `conquer_city()` | `&[&Mob], String` | `()` | Add city if unclaimed |

### Step 5: Update lib.rs [src/lib.rs]

```rust
mod mobs;

// Re-export public items for easier access
pub use mobs::{Mob, Boss, Member, Role};
```

This allows users to write:
```rust
use mobs::{Mob, Boss};  // Instead of mobs::mobs::Mob
```

---

## 🧪 Testing Examples

### Test File Structure

Create `tests/integration_tests.rs`:

```rust
use mobs::{Mob, Boss, Role, Member};

#[test]
fn test_create_mob() {
    let boss = Boss::new("Don Vito", 60);
    let mob = Mob::new("Corleone Family", boss);
    
    assert_eq!(mob.name, "Corleone Family");
    assert_eq!(mob.members.len(), 0);
    assert_eq!(mob.wealth, 0);
}

#[test]
fn test_recruit_member() {
    let boss = Boss::new("Don Vito", 60);
    let mut mob = Mob::new("Corleone Family", boss);
    
    mob.recruit(("Clemenza", 55));
    
    assert_eq!(mob.members.len(), 1);
    assert_eq!(mob.members["Clemenza"].role, Role::Associate);
    assert_eq!(mob.members["Clemenza"].age, 55);
}

#[test]
fn test_attack_higher_power_wins() {
    let boss1 = Boss::new("Don A", 50);
    let boss2 = Boss::new("Don B", 50);
    
    let mut mob1 = Mob::new("Mob A", boss1);
    let mut mob2 = Mob::new("Mob B", boss2);
    
    // Mob 1: 2 Soldiers = 4 power
    mob1.recruit(("Soldier1", 30));
    mob1.recruit(("Soldier2", 35));
    
    // Mob 2: 1 Caporegime = 3 power
    mob2.recruit(("Capo", 45));
    mob2.members.get_mut("Capo").unwrap().role = Role::Caporegime;
    
    let initial_mob2_members = mob2.members.len();
    
    mob1.attack(&mut mob2);
    
    // Mob 2 loses youngest (Capo age 45)
    assert_eq!(mob2.members.len(), initial_mob2_members - 1);
}

#[test]
fn test_steal_wealth() {
    let boss1 = Boss::new("Thief", 40);
    let boss2 = Boss::new("Victim", 40);
    
    let mut thief_mob = Mob::new("Thieves", boss1);
    let mut victim_mob = Mob::new("Victims", boss2);
    
    victim_mob.wealth = 1000;
    thief_mob.steal(&mut victim_mob, 300);
    
    assert_eq!(victim_mob.wealth, 700);
    assert_eq!(thief_mob.wealth, 300);
}

#[test]
fn test_steal_more_than_available() {
    let boss1 = Boss::new("Thief", 40);
    let boss2 = Boss::new("Victim", 40);
    
    let mut thief_mob = Mob::new("Thieves", boss1);
    let mut victim_mob = Mob::new("Victims", boss2);
    
    victim_mob.wealth = 100;
    thief_mob.steal(&mut victim_mob, 500);  // Try to steal 500 but only 100 available
    
    assert_eq!(victim_mob.wealth, 0);
    assert_eq!(thief_mob.wealth, 100);  // Only got 100
}

#[test]
fn test_conquer_city() {
    let boss1 = Boss::new("Conqueror", 50);
    let boss2 = Boss::new("Defender", 50);
    let boss3 = Boss::new("Other", 50);
    
    let mut mob1 = Mob::new("Conquerors", boss1);
    let mob2 = Mob::new("Defenders", boss2);
    let mut mob3 = Mob::new("Others", boss3);
    
    mob2.cities.insert("Rome".to_string());
    
    mob1.conquer_city(&[&mob2, &mob3], "Rome".to_string());
    mob1.conquer_city(&[&mob2, &mob3], "Paris".to_string());
    
    assert!(!mob1.cities.contains("Rome"));   // Can't take Rome
    assert!(mob1.cities.contains("Paris"));   // Can take Paris
}

#[test]
#[should_panic]
fn test_underboss_promotion_panics() {
    let mut member = Member::new(Role::Underboss, 50);
    member.get_promotion();  // Panic!
}
```

Run tests:
```bash
cargo test
```

---

## 🎯 Good vs Bad Code Comparison

### Example 1: Privacy Management

**❌ Bad - Everything Public (Security Risk)**
```rust
pub struct Mob {
    pub members: HashMap<String, Member>,  // Anyone can modify!
}

// User code can do:
mob.members.clear();  // Oops, removed all members!
```

**✅ Good - Hidden Implementation**
```rust
pub struct Mob {
    members: HashMap<String, Member>,  // Private
}

impl Mob {
    pub fn recruit(&mut self, info: (&str, u32)) {
        // Controlled access through method
        let member = Member::new(Role::Associate, info.1);
        self.members.insert(info.0.to_string(), member);
    }
}

// User code must use public method:
mob.recruit(("John", 25));
```

---

### Example 2: Module Organization

**❌ Bad - All in One File**
```rust
// src/lib.rs - 500 lines!
mod boss { /* ... */ }
mod member { /* ... */ }
pub struct Mob { /* ... */ }
impl Mob { /* ... */ }
// Messy and hard to navigate
```

**✅ Good - Organized Files**
```
src/
├── lib.rs           (10 lines - just declares modules)
├── mobs.rs          (Main Mob struct)
└── mobs/
    ├── boss.rs      (Boss struct - separate concern)
    └── member.rs    (Member & Role - separate concern)
```

---

### Example 3: Using Use Statements

**❌ Bad - Full Paths Everywhere**
```rust
let role = crate::mobs::member::Role::Associate;
let member = crate::mobs::member::Member::new(role, 25);
```

**✅ Good - Clean Imports**
```rust
use mobs::{Role, Member};

let role = Role::Associate;
let member = Member::new(role, 25);
```

---

## 🚀 Common Pitfalls & Solutions

| Pitfall | Problem | Solution |
|---------|---------|----------|
| **Forgot `pub mod`** | Submodule not accessible | Add `pub mod member;` in parent |
| **Forgot `pub use`** | Users can't import easily | Re-export: `pub use boss::Boss;` |
| **Private struct fields** | Can't access data | Add getter methods or make fields `pub` |
| **Wrong module path** | Compiler error | Check file structure matches module tree |
| **Circular imports** | Compilation fails | Restructure so no cycles exist |

---

## 📖 Running Examples

### Compile
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Generate Documentation
```bash
cargo doc --open
```

---

## 🎓 Summary Table

What should be in each file:

| File | Purpose | Access Level | Details |
|------|---------|--------------|---------|
| [lib.rs](src/lib.rs) | Crate entry point | Public | Declare `mod mobs` and `pub use` |
| [mobs.rs](src/mobs.rs) | Main Mob struct | Public | Define Mob, declare submodules |
| [mobs/boss.rs](src/mobs/boss.rs) | Boss struct | Public | Boss and `new()` method |
| [mobs/member.rs](src/mobs/member.rs) | Role & Member | Public | Enums and structs with promotion logic |

---

## 🔗 Learning Resources

**Key Concepts Covered:**
- ✅ Package structure with Cargo.toml
- ✅ Library crate (lib.rs) vs Binary crate (main.rs)
- ✅ Module declaration and file structure
- ✅ Public (pub) and private visibility
- ✅ Re-exports with `pub use`
- ✅ Absolute vs relative paths
- ✅ Scope and namespacing
- ✅ Collections (HashMap, HashSet)
- ✅ Enums with associated values
- ✅ Pattern matching (match)
- ✅ Trait derivation (#[derive])

**Next Steps:**
- Add more features (method chaining, traits)
- Create integration tests in `tests/` folder
- Generate and read documentation (`cargo doc --open`)
- Explore workspaces for multiple related crates

---

## 📝 Code Template for New Features

When adding new functionality, follow this pattern:

```rust
// 1. Declare module
pub mod my_feature;

// 2. Create file: src/mobs/my_feature.rs
// 3. Re-export public items
pub use my_feature::MyStruct;

// 4. Implement with clear privacy
#[derive(Debug, PartialEq)]
pub struct MyStruct {
    // pub for public fields, otherwise private
}

impl MyStruct {
    pub fn new(/* params */) -> Self {
        // Public associated function
    }

    pub fn public_method(&self) {
        // Accessible from outside
    }

    fn private_helper(&self) {
        // Only used internally
    }
}
```

---

Happy coding! Remember: **Good organization now = Easy maintenance later** 🚀
