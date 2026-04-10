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
    // ── 04 PatternMatching ──────────────────────────────────────────────────
    Exercise {
        slug: "pm1",
        name: "PatternMatching 1 — case Expressions",
        file: "daml/Exercises/PatternMatching/Ex1.daml",
        hint: "EQ→\"zero\", GT→\"positive\", LT→\"negative\". One-element branch: [_]→\"exactly one element\".",
        section: "04 · PatternMatching",
    },
    Exercise {
        slug: "pm2",
        name: "PatternMatching 2 — Matching on Variants",
        file: "daml/Exercises/PatternMatching/Ex2.daml",
        hint: "colorName: swap Red and Blue. eval: Add uses (+), Neg negates.",
        section: "04 · PatternMatching",
    },
    Exercise {
        slug: "pm3",
        name: "PatternMatching 3 — Tuple and Wildcard Patterns",
        file: "daml/Exercises/PatternMatching/Ex3.daml",
        hint: "myFst: bind x not y. mySnd: bind y not x. classify2D: Q1=(+,+), Q2=(-,+), Q3=(-,-), Q4=(+,-).",
        section: "04 · PatternMatching",
    },
    Exercise {
        slug: "pm4",
        name: "PatternMatching 4 — List Patterns",
        file: "daml/Exercises/PatternMatching/Ex4.daml",
        hint: "myHead: return x not rest. mySum: (+) not (*). safeHead: []→None, (x::_)→Some x.",
        section: "04 · PatternMatching",
    },
    // ── 05 Lists ────────────────────────────────────────────────────────────
    Exercise {
        slug: "lists1",
        name: "Lists 1 — Constructing Lists",
        file: "daml/Exercises/Lists/Ex1.daml",
        hint: "Cons: `0 :: xs`. Concat: `<>`. Fix: length=3, null xs=False, null []=True.",
        section: "05 · Lists",
    },
    Exercise {
        slug: "lists2",
        name: "Lists 2 — map",
        file: "daml/Exercises/Lists/Ex2.daml",
        hint: "doubled:*2. squared:x*x. asText:show x. gtThree:x>3.",
        section: "05 · Lists",
    },
    Exercise {
        slug: "lists3",
        name: "Lists 3 — filter",
        file: "daml/Exercises/Lists/Ex3.daml",
        hint: "Flip each predicate: odd→even, <=5→>5, ==0→/=0, <0→>0.",
        section: "05 · Lists",
    },
    Exercise {
        slug: "lists4",
        name: "Lists 4 — foldl",
        file: "daml/Exercises/Lists/Ex4.daml",
        hint: "total: initial 0. product: (*). maxVal: `if x>acc then x else acc`, start `head nums`. reversed: `x :: acc`.",
        section: "05 · Lists",
    },
    Exercise {
        slug: "lists5",
        name: "Lists 5 — DA.List Utilities",
        file: "daml/Exercises/Lists/Ex5.daml",
        hint: "nub→[1,2,3,4]. sort→ascending. zip→[(1,\"a\")...]. intercalate separator: \", \".",
        section: "05 · Lists",
    },
    // ── 06 Optional ─────────────────────────────────────────────────────────
    Exercise {
        slug: "optional1",
        name: "Optional 1 — Some, None, and fromOptional",
        file: "daml/Exercises/Optional/Ex1.daml",
        hint: "safeDivide: None when b==0.0. Flip the four isSome/isNone booleans. fromOptional default opt.",
        section: "06 · Optional",
    },
    Exercise {
        slug: "optional2",
        name: "Optional 2 — Pattern Matching on Optional",
        file: "daml/Exercises/Optional/Ex2.daml",
        hint: "describe: None→\"nothing\", Some n→\"got \"<>show n. orElse': None→fallback, Some x→x.",
        section: "06 · Optional",
    },
    Exercise {
        slug: "optional3",
        name: "Optional 3 — mapOptional and catOptionals",
        file: "daml/Exercises/Optional/Ex3.daml",
        hint: "keepPositives: x>0. safeReciprocal: None for 0.0. catOptionals expected: [1,2,3].",
        section: "06 · Optional",
    },
    // ── 07 Variants ─────────────────────────────────────────────────────────
    Exercise {
        slug: "variants1",
        name: "Variants 1 — Simple Enumerations",
        file: "daml/Exercises/Variants/Ex1.daml",
        hint: "opposite: North↔South, East↔West. nextLight: Red→Green→Amber→Red.",
        section: "07 · Variants",
    },
    Exercise {
        slug: "variants2",
        name: "Variants 2 — Constructors with Payload",
        file: "daml/Exercises/Variants/Ex2.daml",
        hint: "Circle: 3.14159*r*r. Rectangle: w*h. Triangle: 0.5*b*h.",
        section: "07 · Variants",
    },
    Exercise {
        slug: "variants3",
        name: "Variants 3 — Modelling Domain States",
        file: "daml/Exercises/Variants/Ex3.daml",
        hint: "canShip: only Processing→True. statusLabel: Pending→\"Pending\", Delivered→\"Delivered\".",
        section: "07 · Variants",
    },
    Exercise {
        slug: "variants4",
        name: "Variants 4 — Variants with Record Payloads",
        file: "daml/Exercises/Variants/Ex4.daml",
        hint: "paymentAmount: always use `.amount`. describe: BankTransfer→bank text, CardPayment→card text.",
        section: "07 · Variants",
    },
    // ── 08 Functional101 ────────────────────────────────────────────────────
    Exercise {
        slug: "fp1",
        name: "Functional101 1 — Function Composition",
        file: "daml/Exercises/Functional101/Ex1.daml",
        hint: "addOneThenDouble: `(*2) . (+1)` (rightmost runs first). pipeline: `map (*2)` not `map (^2)`.",
        section: "08 · Functional101",
    },
    Exercise {
        slug: "fp2",
        name: "Functional101 2 — The $ Operator",
        file: "daml/Exercises/Functional101/Ex2.daml",
        hint: "`f (g x)` becomes `f $ g x`. Replace outermost parens with `$`.",
        section: "08 · Functional101",
    },
    Exercise {
        slug: "fp3",
        name: "Functional101 3 — flip, const, and Point-Free Style",
        file: "daml/Exercises/Functional101/Ex3.daml",
        hint: "divideBy: `flip (/) d`. alwaysZero: `const 0`. countWhere: `length . filter p` (drop xs).",
        section: "08 · Functional101",
    },
];

/// Look up an exercise by slug. Returns `None` if not found.
pub fn find(slug: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|e| e.slug == slug)
}
