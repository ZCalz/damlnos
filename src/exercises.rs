/// A single Damlers exercise.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Short identifier used in CLI commands (e.g. "intro1").
    pub slug: &'static str,
    /// Human-readable name shown in `damlers list`.
    pub name: &'static str,
    /// Path to the exercise file, relative to the project root.
    pub file: &'static str,
    /// One-line hint shown by `damlers hint`.
    pub hint: &'static str,
    /// Section label for grouping in `damlers list`.
    pub section: &'static str,
}

/// Ordered list of all exercises. Add new entries here as modules are written.
pub const EXERCISES: &[Exercise] = &[
    // ── 00 Intro ────────────────────────────────────────────────────────────
    Exercise {
        slug: "intro1",
        name: "Intro 1 — Hello, DAML!",
        file: "daml/Exercises/Intro/Ex1.daml",
        hint: "Change `assert False` to `assert True` at the bottom of the script.",
        section: "00 · Intro",
    },
    Exercise {
        slug: "intro2",
        name: "Intro 2 — Assertions",
        file: "daml/Exercises/Intro/Ex2.daml",
        hint: "What does `1 + 1` evaluate to? Fix the number on the right-hand side of `==`.",
        section: "00 · Intro",
    },
    // ── 01 Types ────────────────────────────────────────────────────────────
    Exercise {
        slug: "types1",
        name: "Types 1 — Primitive Literals",
        file: "daml/Exercises/Types/Ex1.daml",
        hint: "Text uses double quotes, Int is a whole number, Decimal needs a decimal point (e.g. 3.14), Bool is True or False.",
        section: "01 · Types",
    },
    Exercise {
        slug: "types2",
        name: "Types 2 — Int vs Decimal",
        file: "daml/Exercises/Types/Ex2.daml",
        hint: "7.0 / 2.0 is a Decimal expression. Use `intToDecimal` to convert an Int to Decimal.",
        section: "01 · Types",
    },
    Exercise {
        slug: "types3",
        name: "Types 3 — Text Operations",
        file: "daml/Exercises/Types/Ex3.daml",
        hint: "Concatenate Text with `<>` not `+`. Use `show` to convert an Int to Text.",
        section: "01 · Types",
    },
    Exercise {
        slug: "types4",
        name: "Types 4 — Party",
        file: "daml/Exercises/Types/Ex4.daml",
        hint: "Use `allocateParty \"Bob\"` to create a new party and bind it with `<-`.",
        section: "01 · Types",
    },
    // ── 02 Records ──────────────────────────────────────────────────────────
    Exercise {
        slug: "records1",
        name: "Records 1 — Defining a Record Type",
        file: "daml/Exercises/Records/Ex1.daml",
        hint: "Both record definitions are missing the `with` keyword after the constructor name.",
        section: "02 · Records",
    },
    Exercise {
        slug: "records2",
        name: "Records 2 — Field Access",
        file: "daml/Exercises/Records/Ex2.daml",
        hint: "Check the `data Book` definition — the field names in the assertions don't match.",
        section: "02 · Records",
    },
    Exercise {
        slug: "records3",
        name: "Records 3 — Record Update Syntax",
        file: "daml/Exercises/Records/Ex3.daml",
        hint: "Use `stock with quantity = stock.quantity + 50` and `stock with price = stock.price * 0.9`.",
        section: "02 · Records",
    },
    Exercise {
        slug: "records4",
        name: "Records 4 — Deriving Eq and Show",
        file: "daml/Exercises/Records/Ex4.daml",
        hint: "Add `deriving (Eq, Show)` indented under the last field of each record's `with` block.",
        section: "02 · Records",
    },
    // ── 03 Functions ────────────────────────────────────────────────────────
    Exercise {
        slug: "functions1",
        name: "Functions 1 — Definitions and Type Signatures",
        file: "daml/Exercises/Functions/Ex1.daml",
        hint: "square: use `*` not `+`. hypotenuse: use `sqrt (a*a + b*b)`. clamp: add guards for lo and hi.",
        section: "03 · Functions",
    },
    Exercise {
        slug: "functions2",
        name: "Functions 2 — if / then / else",
        file: "daml/Exercises/Functions/Ex2.daml",
        hint: "The `then` and `else` branches are swapped in each function — swap them back.",
        section: "03 · Functions",
    },
    Exercise {
        slug: "functions3",
        name: "Functions 3 — Guards",
        file: "daml/Exercises/Functions/Ex3.daml",
        hint: "Fix the result strings in `classify`, and replace `???` with `otherwise` in `letterGrade`.",
        section: "03 · Functions",
    },
    Exercise {
        slug: "functions4",
        name: "Functions 4 — let / in and where",
        file: "daml/Exercises/Functions/Ex4.daml",
        hint: "pi = 3.14159, baseArea = circleArea r, and swap sideA/sideB back to a and b.",
        section: "03 · Functions",
    },
    Exercise {
        slug: "functions5",
        name: "Functions 5 — Higher-Order Functions",
        file: "daml/Exercises/Functions/Ex5.daml",
        hint: "applyTwice: add one more `f` application. applyN: recurse with `f (applyN (n-1) f x)`. Lambda: change `+` to `*`.",
        section: "03 · Functions",
    },
];

/// Look up an exercise by slug. Returns `None` if not found.
pub fn find(slug: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|e| e.slug == slug)
}
