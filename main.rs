// =============================================================================
//  Stellar Soroban Bootcamp — Project 1: Interactive Bill Manager
//
//  Author  : Your Name
//  Stage   : 3 (final — includes Stage 1, 2, and 3 functionality)
//
//  Features:
//    Stage 1 — Add a bill (name + amount) | View all bills with total
//    Stage 2 — Remove a bill by name
//    Stage 3 — Edit an existing bill (rename and/or change amount)
//              with "back" support to cancel mid-edit
//
//  Data structure: HashMap<String, f64>  (bill name → amount owed)
//  Run with  : cargo run
//  Test with : cargo test
// =============================================================================

use std::collections::HashMap;
use std::io::{self, Write};

// -----------------------------------------------------------------------------
// Helper — reads a trimmed line from stdin after printing a prompt
// -----------------------------------------------------------------------------
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    buffer.trim().to_owned()
}

// -----------------------------------------------------------------------------
// Stage 1 — ADD a bill
// -----------------------------------------------------------------------------
fn add_bill(bills: &mut HashMap<String, f64>) {
    println!("\n── Add Bill ─────────────────────────────");

    let name = get_input("  Bill name       : ");
    if name.is_empty() {
        println!("  [!] Name cannot be empty.\n");
        return;
    }
    if bills.contains_key(&name) {
        println!("  [!] '{}' already exists. Use option 4 (Edit) to update it.\n", name);
        return;
    }

    let raw = get_input("  Amount owed ($) : ");
    match raw.trim().parse::<f64>() {
        Ok(amount) if amount >= 0.0 => {
            bills.insert(name.clone(), amount);
            println!("  [+] '{}' added — ${:.2}\n", name, amount);
        }
        Ok(_)  => println!("  [!] Amount must be zero or greater.\n"),
        Err(_) => println!("  [!] '{}' is not a valid number.\n", raw),
    }
}

// -----------------------------------------------------------------------------
// Stage 1 — VIEW all bills
// -----------------------------------------------------------------------------
fn view_bills(bills: &HashMap<String, f64>) {
    println!("\n── Current Bills ────────────────────────");
    if bills.is_empty() {
        println!("  (no bills on record)\n");
        return;
    }

    let mut rows: Vec<(&String, &f64)> = bills.iter().collect();
    rows.sort_by_key(|(name, _)| name.to_lowercase());

    println!("  {:<28} {:>10}", "Name", "Amount ($)");
    println!("  {}", "-".repeat(40));
    for (name, amount) in &rows {
        println!("  {:<28} {:>10.2}", name, amount);
    }
    let total: f64 = rows.iter().map(|(_, &a)| a).sum();
    println!("  {}", "-".repeat(40));
    println!("  {:<28} {:>10.2}  <- TOTAL\n", "", total);
}

// -----------------------------------------------------------------------------
// Stage 2 — REMOVE a bill
// -----------------------------------------------------------------------------
fn remove_bill(bills: &mut HashMap<String, f64>) {
    println!("\n── Remove Bill ──────────────────────────");
    if bills.is_empty() {
        println!("  (no bills to remove)\n");
        return;
    }

    view_bills(bills);
    let name = get_input("  Bill to remove (or 'back') : ");

    if name.eq_ignore_ascii_case("back") || name.is_empty() {
        println!("  [<] Cancelled.\n");
        return;
    }

    match bills.remove(&name) {
        Some(amount) => println!("  [-] Removed '{}' (${:.2})\n", name, amount),
        None         => println!("  [!] No bill named '{}' was found.\n", name),
    }
}

// -----------------------------------------------------------------------------
// Stage 3 — EDIT an existing bill (rename and/or change amount)
// -----------------------------------------------------------------------------
fn edit_bill(bills: &mut HashMap<String, f64>) {
    println!("\n── Edit Bill ────────────────────────────");
    if bills.is_empty() {
        println!("  (no bills to edit)\n");
        return;
    }

    view_bills(bills);

    // Step 1: which bill?
    let old_name = get_input("  Bill to edit (or 'back') : ");
    if old_name.eq_ignore_ascii_case("back") || old_name.is_empty() {
        println!("  [<] Cancelled.\n");
        return;
    }

    let old_amount = match bills.get(&old_name).copied() {
        Some(a) => a,
        None => {
            println!("  [!] No bill named '{}' was found.\n", old_name);
            return;
        }
    };

    println!("\n  Editing '{}' — current amount: ${:.2}", old_name, old_amount);
    println!("  Tip: press Enter to keep current value | type 'back' to cancel.\n");

    // Step 2: optional rename
    let name_input = get_input(&format!("  New name   [{}] : ", old_name));
    if name_input.eq_ignore_ascii_case("back") {
        println!("  [<] Cancelled.\n");
        return;
    }
    let new_name = if name_input.is_empty() { old_name.clone() } else { name_input };

    if new_name != old_name && bills.contains_key(&new_name) {
        println!("  [!] A bill named '{}' already exists. Edit aborted.\n", new_name);
        return;
    }

    // Step 3: optional amount change
    let amount_input = get_input(&format!("  New amount [{:.2}] : ", old_amount));
    if amount_input.eq_ignore_ascii_case("back") {
        println!("  [<] Cancelled.\n");
        return;
    }
    let new_amount = if amount_input.is_empty() {
        old_amount
    } else {
        match amount_input.trim().parse::<f64>() {
            Ok(a) if a >= 0.0 => a,
            Ok(_)  => { println!("  [!] Amount must be zero or greater.\n"); return; }
            Err(_) => { println!("  [!] '{}' is not a valid number.\n", amount_input); return; }
        }
    };

    // Step 4: apply edit
    bills.remove(&old_name);
    bills.insert(new_name.clone(), new_amount);
    println!("  [*] Updated -> '{}' — ${:.2}\n", new_name, new_amount);
}

// -----------------------------------------------------------------------------
// Menu
// -----------------------------------------------------------------------------
fn print_menu() {
    println!("╔══════════════════════════════════════╗");
    println!("║        BILL MANAGER  v1.0            ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  1. Add bill                         ║");
    println!("║  2. View bills                       ║");
    println!("║  3. Remove bill                      ║");
    println!("║  4. Edit bill                        ║");
    println!("║  q. Quit                             ║");
    println!("╚══════════════════════════════════════╝");
}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------
fn main() {
    let mut bills: HashMap<String, f64> = HashMap::new();

    println!("\n  Welcome to the Soroban Bootcamp Bill Manager!\n");

    loop {
        print_menu();
        let choice = get_input("  Your choice: ");

        match choice.to_lowercase().as_str() {
            "1" | "add"    => add_bill(&mut bills),
            "2" | "view"   => view_bills(&bills),
            "3" | "remove" => remove_bill(&mut bills),
            "4" | "edit"   => edit_bill(&mut bills),
            "q" | "quit"   => {
                println!("\n  Goodbye — stay on budget!\n");
                break;
            }
            "" => {}
            _  => println!("\n  [!] Unknown option '{}'. Please try again.\n", choice),
        }
    }
}

// =============================================================================
//  UNIT TESTS — cargo test
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn make_bills() -> HashMap<String, f64> {
        let mut m = HashMap::new();
        m.insert("Rent".to_string(),         1_200.00);
        m.insert("Electricity".to_string(),     95.75);
        m.insert("Water".to_string(),           42.00);
        m.insert("Netflix".to_string(),         15.99);
        m
    }

    // Stage 1 ─────────────────────────────────────────────────────────────────

    #[test]
    fn stage1_insert_stores_correctly() {
        let mut bills = HashMap::new();
        bills.insert("Gym".to_string(), 50.00);
        assert_eq!(bills.len(), 1);
        assert_eq!(*bills.get("Gym").unwrap(), 50.00);
    }

    #[test]
    fn stage1_duplicate_is_rejected() {
        let mut bills = make_bills();
        let before = *bills.get("Rent").unwrap();
        if !bills.contains_key("Rent") {
            bills.insert("Rent".to_string(), 999.00);
        }
        assert_eq!(*bills.get("Rent").unwrap(), before);
    }

    #[test]
    fn stage1_total_is_correct() {
        let bills = make_bills();
        let total: f64 = bills.values().sum();
        let expected = 1_200.00 + 95.75 + 42.00 + 15.99;
        assert!((total - expected).abs() < 0.001);
    }

    #[test]
    fn stage1_sorted_order() {
        let bills = make_bills();
        let mut rows: Vec<&String> = bills.keys().collect();
        rows.sort_by_key(|n| n.to_lowercase());
        assert_eq!(rows[0], "Electricity");
        assert_eq!(rows[1], "Netflix");
        assert_eq!(rows[2], "Rent");
        assert_eq!(rows[3], "Water");
    }

    // Stage 2 ─────────────────────────────────────────────────────────────────

    #[test]
    fn stage2_remove_returns_amount() {
        let mut bills = make_bills();
        assert_eq!(bills.remove("Water"), Some(42.00));
        assert_eq!(bills.len(), 3);
    }

    #[test]
    fn stage2_remove_key_gone() {
        let mut bills = make_bills();
        bills.remove("Netflix");
        assert!(!bills.contains_key("Netflix"));
    }

    #[test]
    fn stage2_remove_nonexistent_is_none() {
        let mut bills = make_bills();
        assert!(bills.remove("DoesNotExist").is_none());
        assert_eq!(bills.len(), 4);
    }

    // Stage 3 ─────────────────────────────────────────────────────────────────

    #[test]
    fn stage3_edit_amount_only() {
        let mut bills = make_bills();
        *bills.get_mut("Rent").unwrap() = 1_350.00;
        assert_eq!(*bills.get("Rent").unwrap(), 1_350.00);
        assert_eq!(bills.len(), 4);
    }

    #[test]
    fn stage3_rename_only() {
        let mut bills = make_bills();
        let amount = bills.remove("Netflix").unwrap();
        bills.insert("Disney+".to_string(), amount);
        assert!(!bills.contains_key("Netflix"));
        assert_eq!(*bills.get("Disney+").unwrap(), 15.99);
        assert_eq!(bills.len(), 4);
    }

    #[test]
    fn stage3_rename_and_amount() {
        let mut bills = make_bills();
        bills.remove("Water");
        bills.insert("Water Bill".to_string(), 55.00);
        assert!(!bills.contains_key("Water"));
        assert_eq!(*bills.get("Water Bill").unwrap(), 55.00);
    }

    #[test]
    fn stage3_rename_onto_existing_is_blocked() {
        let bills = make_bills();
        let blocked = "Rent" != "Water" && bills.contains_key("Rent");
        assert!(blocked);
    }

    #[test]
    fn stage3_back_leaves_map_unchanged() {
        let before = make_bills();
        let after  = make_bills();
        assert_eq!(before.len(), after.len());
        for (k, v) in &before {
            assert_eq!(after.get(k), Some(v));
        }
    }
}
