# Soroban Bootcamp — Project 1: Interactive Bill Manager

A fully interactive command-line bill/expense manager written in Rust.
Implements all three stages of the project spec.

---

## Setup & Run

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
rustc --version   # should print rustc 1.7x.x
cargo --version   # should print cargo 1.7x.x
```

### Run the program
```bash
cargo run
```

### Check for errors (between stages)
```bash
cargo check
```

### Run all unit tests
```bash
cargo test
```

### Run tests with output
```bash
cargo test -- --nocapture
```

---

## Features by Stage

### Stage 1 — Add & View Bills
- Add a bill with a **name** and **amount owed**
- Duplicate names are rejected (user is directed to Edit)
- Invalid or negative amounts are rejected with a clear message
- View all bills sorted **alphabetically** with a running **TOTAL**

### Stage 2 — Remove Bills
- Shows the current bill list before prompting
- Remove a bill by name; reports the amount that was removed
- Type `back` to cancel and return to the main menu

### Stage 3 — Edit Bills
- Edit name, amount, or both on an existing bill
- Press **Enter** to keep the current value for any field
- Type `back` at any prompt to cancel the entire edit
- Prevents renaming onto an already-existing bill name

---

## Project Structure

```
bill_manager/
├── Cargo.toml        ← project manifest (no external dependencies)
├── README.md         ← this file
└── src/
    └── main.rs       ← all application code + 12 unit tests
```

---

## Design Decisions

### Why HashMap instead of Vec?

| Operation | `Vec<(String,f64)>` | `HashMap<String,f64>` |
|-----------|--------------------|-----------------------|
| Lookup by name | O(n) | O(1) average |
| Remove by name | O(n) | O(1) average |
| Edit by name   | O(n) | O(1) average |
| Iterate all    | O(n) | O(n) |

`HashMap` is the right choice from Stage 2 onwards because remove and edit operations look up by name. The tip in the project spec agrees: *"a HashMap will be easier to work with at stages 2 and 3."*

### One function per menu action
Each of `add_bill`, `view_bills`, `remove_bill`, and `edit_bill` is a standalone function. This keeps each concern isolated and makes the `loop`/`match` in `main` easy to read.

### Immutable vs mutable borrows
```rust
fn view_bills(bills: &HashMap<String, f64>)      // read-only — &T
fn add_bill  (bills: &mut HashMap<String, f64>)  // mutates  — &mut T
```
The Rust compiler enforces these at compile time — no accidental mutations.

---

## Sample Session

```
  Welcome to the Soroban Bootcamp Bill Manager!

╔══════════════════════════════════════╗
║        BILL MANAGER  v1.0            ║
╠══════════════════════════════════════╣
║  1. Add bill                         ║
║  2. View bills                       ║
║  3. Remove bill                      ║
║  4. Edit bill                        ║
║  q. Quit                             ║
╚══════════════════════════════════════╝
  Your choice: 1

── Add Bill ─────────────────────────────
  Bill name       : Rent
  Amount owed ($) : 1200
  [+] 'Rent' added — $1200.00

  Your choice: 1
  Bill name       : Netflix
  Amount owed ($) : 15.99
  [+] 'Netflix' added — $15.99

  Your choice: 2

── Current Bills ────────────────────────
  Name                           Amount ($)
  ----------------------------------------
  Netflix                             15.99
  Rent                             1200.00
  ----------------------------------------
                                   1215.99  <- TOTAL

  Your choice: 4

── Edit Bill ────────────────────────────
  Bill to edit (or 'back') : Netflix

  Editing 'Netflix' — current amount: $15.99
  Tip: press Enter to keep current value | type 'back' to cancel.

  New name   [Netflix] : Disney+
  New amount [15.99]   : 13.99
  [*] Updated -> 'Disney+' — $13.99

  Your choice: 3
  Bill to remove (or 'back') : Rent
  [-] Removed 'Rent' ($1200.00)

  Your choice: q
  Goodbye — stay on budget!
```

---

## Connection to Stellar / Soroban

Every pattern in this project maps directly to Soroban smart contract development:

| CLI Concept | Soroban Equivalent |
|---|---|
| `HashMap<String, f64>` | `Map<Symbol, i128>` in on-chain storage |
| `&mut bills` passed to each fn | `Env` passed into every `#[contractimpl]` function |
| `match` on menu input | `match` dispatching contract function calls |
| `Result`/`Option` error handling | `Result<(), ContractError>` return types |
| Input validation before insert | `require!` / `panic!` guards in contract functions |

```rust
// The HashMap pattern you used here becomes this in Soroban:
#[contractimpl]
impl BillContract {
    pub fn add_bill(env: Env, name: Symbol, amount: i128) {
        let mut bills: Map<Symbol, i128> = env.storage().instance().get(&KEY).unwrap_or_default();
        if bills.contains_key(name.clone()) {
            panic!("bill already exists");
        }
        bills.set(name, amount);
        env.storage().instance().set(&KEY, &bills);
    }
}
```
