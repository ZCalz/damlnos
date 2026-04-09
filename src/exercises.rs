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
];

/// Look up an exercise by slug. Returns `None` if not found.
pub fn find(slug: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|e| e.slug == slug)
}
