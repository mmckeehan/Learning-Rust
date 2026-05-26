# 🦀 Rust Bootcamp for Beginners
### A 10-Day Guide for New Programmers (Ages 13+)
**Time per day:** 30–60 minutes | **Teacher:** Senior Rust Engineer | **Learner level:** Zero experience

---

> **How to use this checklist:**
> Each day has a short lesson and a task. Check off each box as you go. Don't rush — understanding is more important than speed. It's totally okay to re-read a section or ask questions!

---

## 🧠 Core Concepts Overview
> These are the big ideas you'll learn during the 10 days. Don't worry about knowing them now — just keep this list in mind as you go!

- [ ] What is a program? (Instructions a computer follows)
- [ ] What is a variable? (A box that holds information)
- [ ] What are data types? (What kind of thing is in the box?)
- [ ] What is a function? (A reusable set of instructions)
- [ ] What is control flow? (Making decisions in code with if/else)
- [ ] What is a loop? (Repeating instructions)
- [ ] What is ownership? (Rust's special way of managing memory — unique to Rust!)
- [ ] What is a struct? (A custom "thing" you define, like a character in a game)
- [ ] What is an enum? (A list of possible states or options)
- [ ] What is error handling? (What to do when something goes wrong)

---

## ⚙️ Before You Start — Setup Checklist

- [x] Ask a parent or guardian to help with installation
- [x] Install Rust from **https://rustup.rs** (free, safe, official)
- [x] Open a terminal (Command Prompt on Windows, Terminal on Mac/Linux)
- [x] Type `rustc --version` and press Enter — if you see a version number, you're ready! 🎉
- [x] Install a code editor — **VS Code** is recommended (free at https://code.visualstudio.com)
- [x] Install the **rust-analyzer** extension inside VS Code (helps catch mistakes as you type)

---

## 📅 Day 1 — "Hello, World!" and How Programs Work
**Goal:** Understand what code is and run your very first Rust program.

### 📖 Lesson
- [x] Read: A **program** is a list of instructions you write, and the computer follows them in order — like a recipe.
- [x] Read: **Rust** is a programming language known for being fast and safe. Big companies like Mozilla, Microsoft, and Discord use it.
- [x] Read: Every Rust program starts inside a special function called `main`. Think of it as the "front door" of your program.
- [x] Read: `println!` is how you make Rust print text to the screen. The `!` means it's a *macro* (a special kind of command).

### 💻 Concept in Code
```rust
fn main() {
    println!("Hello, world!");
}
```

### ✅ Tasks
- [x] Create a new folder called `day1` on your computer
- [x] Inside it, create a file called `main.rs`
- [x] Type (don't copy-paste!) the code above into `main.rs`
- [x] In your terminal, navigate to the folder and run: `rustc main.rs`
- [x] Then run: `./main` (Mac/Linux) or `main.exe` (Windows)
- [x] See "Hello, world!" printed? **You just ran your first Rust program!** ✅

### 🏗️ Mini Project #1 — "About Me" Printer
- [x] Change the program to print 3 things about yourself (your name, your age, your favorite hobby)
- [x] Example output:
  ```
  My name is Alex.
  I am 13 years old.
  I love playing video games.
  ```
- [x] Run it and make sure all 3 lines print correctly

---

## 📅 Day 2 — Variables and Data Types
**Goal:** Learn how to store information in your program.

### 📖 Lesson
- [x] Read: A **variable** is like a labeled box — you put information in it and use the label to get it back.
- [x] Read: In Rust, you create a variable with the word `let`. Example: `let age = 13;`
- [x] Read: **Data types** describe what kind of thing is stored:
  - `i32` = a whole number (integer), like `5` or `-3`
  - `f64` = a decimal number, like `3.14`
  - `bool` = true or false
  - `String` = text/words
- [x] Read: By default, variables in Rust **cannot be changed** after you set them. This is called being *immutable*. To allow changes, use `let mut`.

### 💻 Concept in Code
```rust
fn main() {
    let name = "Alex";        // text (string)
    let age = 13;             // whole number
    let height = 5.4;         // decimal number
    let likes_rust = true;    // true or false

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Likes Rust: {}", likes_rust);
}
```

### ✅ Tasks
- [x] Create a `day2` folder and a new `main.rs`
- [x] Type the code above and run it
- [x] Change the values to match your own info
- [x] Add one more variable of your choice and print it

### 🏗️ Mini Project #2 — Simple Calculator (Addition)
- [x] Create two variables: `num1 = 20` and `num2 = 5`
- [x] Create a third variable: `result = num1 + num2`
- [x] Print: `"20 + 5 = 25"` (using your variables)
- [x] Try subtraction (`-`), multiplication (`*`), and division (`/`) too

---

## 📅 Day 3 — Functions
**Goal:** Learn how to write reusable blocks of code.

### 📖 Lesson
- [x] Read: A **function** is a named block of code you can run whenever you need it — like saving a spell in a game to cast later.
- [x] Read: You define a function with `fn`, give it a name, and put instructions inside `{ }`.
- [x] Read: Functions can take **inputs** (called *parameters*) and give back an **output** (called a *return value*).
- [x] Read: In Rust, the last line of a function (without a semicolon) is automatically returned.

### 💻 Concept in Code
```rust
fn greet(name: &str) {
    println!("Hello, {}! Welcome to Rust.", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon = this value is returned
}

fn main() {
    greet("Alex");
    let result = add(10, 5);
    println!("10 + 5 = {}", result);
}
```

### ✅ Tasks
- [ ] Create a `day3` folder and type the code above
- [ ] Call `greet` with your own name
- [ ] Run it and confirm both lines print correctly
- [ ] Add a `multiply(a: i32, b: i32) -> i32` function and call it from main

### 🏗️ Mini Project #3 — Temperature Converter
- [ ] Write a function `celsius_to_fahrenheit(c: f64) -> f64` that converts temperature
  - Formula: `(c * 9.0 / 5.0) + 32.0`
- [ ] In main, convert these temps and print each result: `0°C`, `100°C`, `37°C`
- [ ] Expected output example: `0°C = 32°F`

---

## 📅 Day 4 — If / Else (Making Decisions)
**Goal:** Make your program do different things based on conditions.

### 📖 Lesson
- [ ] Read: An **if statement** lets your program make a decision: "If this is true, do this. Otherwise, do something else."
- [ ] Read: You can chain decisions with `else if`.
- [ ] Read: Comparison operators: `==` (equals), `!=` (not equal), `>` (greater), `<` (less), `>=`, `<=`
- [ ] Read: Logical operators: `&&` means "and", `||` means "or", `!` means "not"

### 💻 Concept in Code
```rust
fn main() {
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F — keep trying!");
    }
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Change `score` to different values (95, 75, 55) and see what prints each time
- [ ] Add an extra `else if` for a score of 60–69 as Grade D

### 🏗️ Mini Project #4 — Number Guessing Judge
- [ ] Create a variable `secret = 42` and a variable `guess = 50`
- [ ] Write if/else logic to print:
  - `"Too high!"` if guess > secret
  - `"Too low!"` if guess < secret
  - `"You got it!"` if they're equal
- [ ] Test all three cases by changing `guess`

---

## 📅 Day 5 — Loops
**Goal:** Repeat code without writing it over and over.

### 📖 Lesson
- [ ] Read: A **loop** repeats a block of code. Instead of writing `println!` 10 times, you use a loop!
- [ ] Read: `loop` repeats forever until you say `break`.
- [ ] Read: `while` repeats as long as a condition is true.
- [ ] Read: `for` loops through a range or a list of items.
- [ ] Read: `break` stops a loop. `continue` skips to the next round.

### 💻 Concept in Code
```rust
fn main() {
    // Count from 1 to 5
    for i in 1..=5 {
        println!("Count: {}", i);
    }

    // While loop
    let mut energy = 3;
    while energy > 0 {
        println!("Energy remaining: {}", energy);
        energy -= 1;
    }
    println!("Out of energy!");
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Modify the for loop to count from 1 to 10
- [ ] Add a `for` loop that prints only even numbers from 2 to 20

### 🏗️ Mini Project #5 — Multiplication Table Printer
- [ ] Ask a number to be stored in a variable (e.g., `let table = 7`)
- [ ] Use a `for` loop to print the multiplication table for that number (1×7 through 10×7)
- [ ] Expected output:
  ```
  7 x 1 = 7
  7 x 2 = 14
  ...
  7 x 10 = 70
  ```

---

## 📅 Day 6 — Ownership (Rust's Superpower)
**Goal:** Understand what makes Rust different from other languages.

### 📖 Lesson
- [ ] Read: Every other programming language has to carefully manage computer memory (where data is stored). Rust does this automatically using a system called **ownership**.
- [ ] Read: The rule is simple: every piece of data has **one owner**. When the owner is done, the data is cleaned up automatically.
- [ ] Read: You can **borrow** data (look at it without taking it) using `&`. This is called a *reference*.
- [ ] Read: If you borrow something, you promise not to delete it while it's being borrowed.
- [ ] Read: This is what makes Rust programs safe — no memory leaks, no crashes from bad memory!

### 💻 Concept in Code
```rust
fn print_name(name: &str) {   // borrowing name with &
    println!("Hello, {}!", name);
}

fn main() {
    let my_name = String::from("Alex");
    print_name(&my_name);     // lending it with &
    println!("Still have my name: {}", my_name); // we still own it!
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Write a second function `shout_name(name: &str)` that prints the name in all caps using `.to_uppercase()`
- [ ] Call both functions in main and confirm the name is still usable after both calls

### 📝 Reflection (write your answers as comments in the code)
- [ ] In your own words: Why is ownership useful?
- [ ] What does `&` do?

---

## 📅 Day 7 — Structs (Building Your Own Data Types)
**Goal:** Group related data together into a custom "blueprint."

### 📖 Lesson
- [ ] Read: A **struct** (short for structure) lets you create a custom type that groups related data together.
- [ ] Read: Think of it like a character sheet in a game — a `Player` has a name, health, and score all in one place.
- [ ] Read: You define a struct with `struct`, then create an *instance* of it (like filling out the sheet).
- [ ] Read: You can add functions that belong to a struct using `impl` (short for implementation).

### 💻 Concept in Code
```rust
struct Player {
    name: String,
    health: i32,
    score: i32,
}

impl Player {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn describe(&self) {
        println!("{} | HP: {} | Score: {}", self.name, self.health, self.score);
    }
}

fn main() {
    let p1 = Player {
        name: String::from("Alex"),
        health: 100,
        score: 500,
    };

    p1.describe();
    println!("Alive: {}", p1.is_alive());
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Create a second player `p2` with different values
- [ ] Add a method `level(&self) -> i32` that returns `self.score / 100` as the player's level
- [ ] Print the level for both players

### 🏗️ Mini Project #6 — Pet Stats Card
- [ ] Create a `Pet` struct with: `name`, `species`, `age`, `is_friendly`
- [ ] Add a method `introduce(&self)` that prints something like: `"I'm Buddy, a 3-year-old Dog. Friendly: true"`
- [ ] Create at least 2 pets and call `introduce()` on each

---

## 📅 Day 8 — Enums and Pattern Matching
**Goal:** Represent a fixed set of options and handle each one precisely.

### 📖 Lesson
- [ ] Read: An **enum** (enumeration) is a type that can be one of several named options.
- [ ] Read: Example: A traffic light can only be `Red`, `Yellow`, or `Green` — that's an enum!
- [ ] Read: **Pattern matching** with `match` lets you handle each possible option differently — like a super-powered if/else.
- [ ] Read: Rust forces you to handle *every* possible case in a `match`. This prevents bugs where you forgot a possibility.

### 💻 Concept in Code
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East  => println!("Heading East!"),
        Direction::West  => println!("Heading West!"),
    }
}

fn main() {
    describe_direction(Direction::North);
    describe_direction(Direction::West);
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Add a fifth direction: `Up` and handle it in the match
- [ ] Notice: Rust won't compile until you handle every case in match — try commenting one out to see the error!

### 🏗️ Mini Project #7 — Simple RPG Combat
- [ ] Create an enum `Action` with variants: `Attack`, `Defend`, `Heal`
- [ ] Write a function `perform_action(action: Action)` that uses `match` to print what happens:
  - `Attack` → `"You strike the enemy for 10 damage!"`
  - `Defend` → `"You raise your shield. Damage reduced!"`
  - `Heal` → `"You drink a potion and recover 15 HP!"`
- [ ] Call all three actions from main

---

## 📅 Day 9 — Vectors and Collections
**Goal:** Store and work with lists of data.

### 📖 Lesson
- [ ] Read: So far, variables hold one thing. A **Vec** (short for Vector) holds a *list* of things.
- [ ] Read: Think of a Vec like a shopping list — you can add items, remove them, and look through them.
- [ ] Read: You create one with `Vec::new()` or `vec![item1, item2, item3]`
- [ ] Read: Common methods: `.push(item)` adds to the end, `.len()` tells you how many items there are, and you can loop through them with `for item in &my_vec`.

### 💻 Concept in Code
```rust
fn main() {
    let mut scores: Vec<i32> = vec![85, 92, 78, 95, 60];

    scores.push(88);  // add a new score

    println!("Total scores: {}", scores.len());

    for score in &scores {
        println!("Score: {}", score);
    }

    // Find the highest score
    let mut highest = scores[0];
    for &score in &scores {
        if score > highest {
            highest = score;
        }
    }
    println!("Highest score: {}", highest);
}
```

### ✅ Tasks
- [ ] Type and run the code above
- [ ] Add a few more scores to the vec and run again
- [ ] Modify the loop to also find and print the **lowest** score

### 🏗️ Mini Project #8 — High Score Tracker
- [ ] Create a Vec of player names: `vec!["Alice", "Bob", "Charlie", "Dana"]`
- [ ] Create a Vec of their scores: `vec![340, 210, 480, 125]`
- [ ] Loop through both using an index (`for i in 0..names.len()`) and print each player's name and score
- [ ] Find and print who has the highest score

---

## 📅 Day 10 — Putting It All Together
**Goal:** Use everything you've learned in two final projects.

### 📖 Lesson — Review
- [ ] Re-read your Day 1–9 notes or comments in your code
- [ ] Concepts you should now understand:
  - [ ] Variables and data types
  - [ ] Functions
  - [ ] If/else and loops
  - [ ] Ownership and borrowing
  - [ ] Structs and impl
  - [ ] Enums and match
  - [ ] Vectors

### 🏗️ Mini Project #9 — Word Counter
- [ ] Create a Vec of words: `vec!["the", "quick", "brown", "fox", "the", "quick", "fox", "fox"]`
- [ ] Count how many times the word `"fox"` appears in the list
- [ ] Print: `"'fox' appears 3 time(s)"`
- [ ] **Bonus:** Also count how many times `"the"` and `"quick"` appear

### 🏗️ Mini Project #10 — Mini Adventure Game
*This is your graduation project! Use structs, enums, functions, loops, and if/else together.*

- [ ] Create a `Hero` struct with: `name: String`, `health: i32`, `gold: i32`
- [ ] Create an `Event` enum with: `FindGold`, `BattleEnemy`, `RestAtInn`
- [ ] Write a function `process_event(hero: &mut Hero, event: Event)` that uses `match`:
  - `FindGold` → adds 10 gold, prints `"You found 10 gold!"`
  - `BattleEnemy` → subtracts 15 health, prints `"You fought an enemy! -15 HP"`
  - `RestAtInn` → adds 20 health (max 100), prints `"You rested. +20 HP"`
- [ ] In `main`, create your hero, then run a sequence of at least 5 events using a Vec
- [ ] After each event, print the hero's current health and gold
- [ ] At the end, print whether the hero survived (health > 0) or not

---

## 🎓 Bootcamp Complete! What's Next?

Congrats — you finished the 10-day Rust Bootcamp! 🦀🎉

### Next Steps
- [ ] **The Rust Book** (free online): https://doc.rust-lang.org/book/ — the official guide, written for beginners
- [ ] **Rustlings** (free interactive exercises): https://github.com/rust-lang/rustlings — small practice challenges
- [ ] **Exercism Rust Track** (free): https://exercism.org/tracks/rust — great for practicing with feedback
- [ ] Build your own idea! Think of a small tool or game and try to make it in Rust

### Skills You've Unlocked
- [x] Set up a real development environment
- [x] Written and run Rust programs from scratch
- [x] Used variables, functions, loops, and conditionals
- [x] Learned Rust's unique ownership model
- [x] Created custom data types with structs and enums
- [x] Worked with collections (Vectors)
- [x] Built 10 projects from scratch

---

*Created for a 10-day beginner Rust bootcamp. Designed for learners age 13+ with zero prior programming experience.*
*Rust version targeted: 1.75+ | All code uses stable Rust features only.*
