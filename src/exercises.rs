/// A single Damlings exercise.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Short identifier used in CLI commands (e.g. "intro1").
    pub slug: &'static str,
    /// Human-readable name shown in `damlings list`.
    pub name: &'static str,
    /// Path to the exercise file, relative to the project root.
    pub file: &'static str,
    /// One-line hint shown by `damlings hint`.
    pub hint: &'static str,
    /// Section label for grouping in `damlings list`.
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
    // ── Quiz 01 ─────────────────────────────────────────────────────────────
    Exercise {
        slug: "quiz1",
        name: "Quiz 01 — Bank Account",
        file: "daml/Exercises/Quiz01/Quiz01.daml",
        hint: "There are no hints for quiz exercises. Read the failing test output and the type definitions in the file.",
        section: "Quiz 01 · Bank Account",
    },
    // ── 09 Templates ────────────────────────────────────────────────────────
    Exercise {
        slug: "templates1",
        name: "Templates 1 — Declaring a Template",
        file: "daml/Exercises/Templates/Ex1.daml",
        hint: "Add `text : Text` to the `with` block. Change `observer author` to `signatory author`.",
        section: "09 · Templates",
    },
    Exercise {
        slug: "templates2",
        name: "Templates 2 — Signatory and Observer",
        file: "daml/Exercises/Templates/Ex2.daml",
        hint: "`signatory [issuer, owner]` (list both). `observer watcher` (not owner).",
        section: "09 · Templates",
    },
    Exercise {
        slug: "templates3",
        name: "Templates 3 — ensure",
        file: "daml/Exercises/Templates/Ex3.daml",
        hint: "Replace `ensure True` with `ensure (amount > 0.0 && description /= \"\")`.",
        section: "09 · Templates",
    },
    Exercise {
        slug: "templates4",
        name: "Templates 4 — Querying the Ledger",
        file: "daml/Exercises/Templates/Ex4.daml",
        hint: "\"Wrong text\" → \"Buy milk\". `length notes == 0` → `length notes == 1`.",
        section: "09 · Templates",
    },
    // ── 10 Choices ──────────────────────────────────────────────────────────
    Exercise {
        slug: "choices1",
        name: "Choices 1 — Consuming Choices",
        file: "daml/Exercises/Choices/Ex1.daml",
        hint: "Remove `nonconsuming`. Change controller to `owner`. Return `newOwner` not `owner`.",
        section: "10 · Choices",
    },
    Exercise {
        slug: "choices2",
        name: "Choices 2 — Nonconsuming Choices",
        file: "daml/Exercises/Choices/Ex2.daml",
        hint: "Add `nonconsuming`. Return type `()` not `ContractId Counter`. Controller `anyone` not `owner`.",
        section: "10 · Choices",
    },
    Exercise {
        slug: "choices3",
        name: "Choices 3 — Choice with Return Value",
        file: "daml/Exercises/Choices/Ex3.daml",
        hint: "Add `nonconsuming` to GetBalance. Return `balance` not `0.0`.",
        section: "10 · Choices",
    },
    Exercise {
        slug: "choices4",
        name: "Choices 4 — Chaining Choices",
        file: "daml/Exercises/Choices/Ex4.daml",
        hint: "Remove `nonconsuming` from Split. Second create: `amount = rightAmount` not `amount`.",
        section: "10 · Choices",
    },
    // ── 11 Scripts ──────────────────────────────────────────────────────────
    Exercise {
        slug: "scripts1",
        name: "Scripts 1 — submit vs submitMustFail",
        file: "daml/Exercises/Scripts/Ex1.daml",
        hint: "Authorized create: `submit`. Unauthorized create: `submitMustFail`.",
        section: "11 · Scripts",
    },
    Exercise {
        slug: "scripts2",
        name: "Scripts 2 — allocateParty and getTime",
        file: "daml/Exercises/Scripts/Ex2.daml",
        hint: "`pure alice` → `allocateParty \"Carol\"`. Time assertion: `/=` → `==`.",
        section: "11 · Scripts",
    },
    Exercise {
        slug: "scripts3",
        name: "Scripts 3 — Multi-party Transactions",
        file: "daml/Exercises/Scripts/Ex3.daml",
        hint: "Add `alice` to the `submitMulti` signer list. Repay controlled by `borrower` (alice).",
        section: "11 · Scripts",
    },
    // ── 12 Parties ──────────────────────────────────────────────────────────
    Exercise {
        slug: "parties1",
        name: "Parties 1 — Visibility and Privacy",
        file: "daml/Exercises/Parties/Ex1.daml",
        hint: "Alice (signatory) sees it → `/= None`. Bob (non-stakeholder) cannot → `== None`.",
        section: "12 · Parties",
    },
    Exercise {
        slug: "parties2",
        name: "Parties 2 — Adding an Observer",
        file: "daml/Exercises/Parties/Ex2.daml",
        hint: "`observer owner` → `observer reader` so Bob can see the contract.",
        section: "12 · Parties",
    },
    Exercise {
        slug: "parties3",
        name: "Parties 3 — Divulgence via fetch",
        file: "daml/Exercises/Parties/Ex3.daml",
        hint: "`fetch otherCid` → `fetch secretCid`. Return `s` not `()`.",
        section: "12 · Parties",
    },
    // ── 13 Constraints ──────────────────────────────────────────────────────
    Exercise {
        slug: "constraints1",
        name: "Constraints 1 — assert inside a Choice",
        file: "daml/Exercises/Constraints/Ex1.daml",
        hint: "`amount <= 0.0` → `amount > 0.0` to allow positive withdrawals.",
        section: "13 · Constraints",
    },
    Exercise {
        slug: "constraints2",
        name: "Constraints 2 — abortMsg",
        file: "daml/Exercises/Constraints/Ex2.daml",
        hint: "Guard: `status == \"Pending\"` → `status /= \"Pending\"`. New status: `\"Pending\"` → `\"Shipped\"`.",
        section: "13 · Constraints",
    },
    // ── Quiz 02 ─────────────────────────────────────────────────────────────
    Exercise {
        slug: "quiz2",
        name: "Quiz 02 — IOU Contract",
        file: "daml/Exercises/Quiz02/Quiz02.daml",
        hint: "There are no hints for quiz exercises. Read the failing test output and the template definitions in the file.",
        section: "Quiz 02 · IOU Contract",
    },
    // ── 14 Keys ─────────────────────────────────────────────────────────────
    Exercise {
        slug: "keys1",
        name: "Keys 1 — Contract Keys",
        file: "daml/Exercises/Keys/Ex1.daml",
        hint: "`key owner : Party` → `key (owner, code) : (Party, Text)`. `maintainer key` → `maintainer key._1`.",
        section: "14 · Keys",
    },
    Exercise {
        slug: "keys2",
        name: "Keys 2 — exerciseByKey",
        file: "daml/Exercises/Keys/Ex2.daml",
        hint: "Controller: `maintainer` → `owner`. Key arg: `\"Alice\"` → `(alice, \"Alice\")`.",
        section: "14 · Keys",
    },
    // ── 15 ProposeAccept ────────────────────────────────────────────────────
    Exercise {
        slug: "pa1",
        name: "ProposeAccept 1 — Propose-Accept Pattern",
        file: "daml/Exercises/ProposeAccept/Ex1.daml",
        hint: "Accept controller: `receiver` not `proposer`. Recipient field: `receiver` not `proposer`.",
        section: "15 · ProposeAccept",
    },
    Exercise {
        slug: "pa2",
        name: "ProposeAccept 2 — Counter-offer",
        file: "daml/Exercises/ProposeAccept/Ex2.daml",
        hint: "CounterOffer controller: `receiver`. New proposal: `proposer = receiver, receiver = proposer`.",
        section: "15 · ProposeAccept",
    },
    // ── 16 Compose ──────────────────────────────────────────────────────────
    Exercise {
        slug: "compose1",
        name: "Compose 1 — Atomic Swap",
        file: "daml/Exercises/Compose/Ex1.daml",
        hint: "Second Transfer: `newOwner = owner1` not `owner2`.",
        section: "16 · Compose",
    },
    // ── 17 Interfaces ───────────────────────────────────────────────────────
    Exercise {
        slug: "interfaces1",
        name: "Interfaces 1 — Implementing an Interface",
        file: "daml/Exercises/Interfaces/Ex1.daml",
        hint: "`getOwner = owner` (not issuer). `getAmount = amount` (not 0.0).",
        section: "17 · Interfaces",
    },
    Exercise {
        slug: "interfaces2",
        name: "Interfaces 2 — Interface Choices",
        file: "daml/Exercises/Interfaces/Ex2.daml",
        hint: "Return `\"Hello from \" <> name` instead of `\"Hello from nobody\"`.",
        section: "17 · Interfaces",
    },
    // ── 18 Exceptions ───────────────────────────────────────────────────────
    Exercise {
        slug: "exceptions1",
        name: "Exceptions 1 — throw and catch",
        file: "daml/Exercises/Exceptions/Ex1.daml",
        hint: "Add `deriving Show` to InsufficientFunds. Flip guard: `amount <= balance` → `amount > balance`.",
        section: "18 · Exceptions",
    },
    // ── Quiz 03 ─────────────────────────────────────────────────────────────
    Exercise {
        slug: "quiz3",
        name: "Quiz 03 — Token Swap with Interface",
        file: "daml/Exercises/Quiz03/Quiz03.daml",
        hint: "There are no hints for quiz exercises. Read the failing test output and the interface/template definitions in the file.",
        section: "Quiz 03 · Token Swap",
    },
];

/// Look up an exercise by slug. Returns `None` if not found.
pub fn find(slug: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|e| e.slug == slug)
}
