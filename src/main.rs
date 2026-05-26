use std::collections::HashMap;
use std::io::{self, Write};

fn get_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_owned()
}

fn add_bill_data(bills: &mut HashMap<String, f64>, name: String, amount: f64) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name cannot be empty.".to_string());
    }
    if amount < 0.0 {
        return Err("Amount must be zero or greater.".to_string());
    }
    if bills.contains_key(&name) {
        return Err(format!(
            "'{}' already exists. Use edit to update it.",
            name
        ));
    }
    bills.insert(name, amount);
    Ok(())
}

fn remove_bill_data(bills: &mut HashMap<String, f64>, name: &str) -> Option<f64> {
    bills.remove(name)
}

fn edit_bill_data(
    bills: &mut HashMap<String, f64>,
    old_name: &str,
    new_name: String,
    new_amount: f64,
) -> Result<(), String> {
    if new_name.is_empty() {
        return Err("Name cannot be empty.".to_string());
    }
    if new_amount < 0.0 {
        return Err("Amount must be zero or greater.".to_string());
    }
    if !bills.contains_key(old_name) {
        return Err(format!("No bill named '{}' was found.", old_name));
    }
    if old_name != new_name && bills.contains_key(&new_name) {
        return Err(format!("A bill named '{}' already exists.", new_name));
    }

    bills.remove(old_name);
    bills.insert(new_name, new_amount);
    Ok(())
}

fn print_bill_rows(bills: &HashMap<String, f64>) {
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
    let total: f64 = rows.iter().map(|(_, &amount)| amount).sum();
    println!("  {}", "-".repeat(40));
    println!("  {:<28} {:>10.2}  <- TOTAL\n", "", total);
}

fn add_bill(bills: &mut HashMap<String, f64>) {
    println!("\n-- Add Bill --");
    let name = get_input("  Bill name       : ");
    let amount_raw = get_input("  Amount owed ($) : ");
    let amount = match amount_raw.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("  [!] '{}' is not a valid number.\n", amount_raw);
            return;
        }
    };

    match add_bill_data(bills, name.clone(), amount) {
        Ok(()) => println!("  [+] '{}' added - ${:.2}\n", name, amount),
        Err(message) => println!("  [!] {message}\n"),
    }
}

fn view_bills(bills: &HashMap<String, f64>) {
    println!("\n-- Current Bills --");
    print_bill_rows(bills);
}

fn remove_bill(bills: &mut HashMap<String, f64>) {
    println!("\n-- Remove Bill --");
    if bills.is_empty() {
        println!("  (no bills to remove)\n");
        return;
    }

    print_bill_rows(bills);
    let name = get_input("  Bill to remove (or 'back') : ");
    if name.eq_ignore_ascii_case("back") || name.is_empty() {
        println!("  [<] Cancelled.\n");
        return;
    }

    match remove_bill_data(bills, &name) {
        Some(amount) => println!("  [-] Removed '{}' (${:.2})\n", name, amount),
        None => println!("  [!] No bill named '{}' was found.\n", name),
    }
}

fn edit_bill(bills: &mut HashMap<String, f64>) {
    println!("\n-- Edit Bill --");
    if bills.is_empty() {
        println!("  (no bills to edit)\n");
        return;
    }

    print_bill_rows(bills);
    let old_name = get_input("  Bill to edit (or 'back') : ");
    if old_name.eq_ignore_ascii_case("back") || old_name.is_empty() {
        println!("  [<] Cancelled.\n");
        return;
    }

    let old_amount = match bills.get(&old_name).copied() {
        Some(amount) => amount,
        None => {
            println!("  [!] No bill named '{}' was found.\n", old_name);
            return;
        }
    };

    println!("\n  Editing '{}' - current amount: ${:.2}", old_name, old_amount);
    println!("  Press Enter to keep current value, or type 'back' to cancel.\n");

    let name_input = get_input(&format!("  New name   [{}] : ", old_name));
    if name_input.eq_ignore_ascii_case("back") {
        println!("  [<] Cancelled.\n");
        return;
    }
    let new_name = if name_input.is_empty() {
        old_name.clone()
    } else {
        name_input
    };

    let amount_input = get_input(&format!("  New amount [{:.2}] : ", old_amount));
    if amount_input.eq_ignore_ascii_case("back") {
        println!("  [<] Cancelled.\n");
        return;
    }
    let new_amount = if amount_input.is_empty() {
        old_amount
    } else {
        match amount_input.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("  [!] '{}' is not a valid number.\n", amount_input);
                return;
            }
        }
    };

    match edit_bill_data(bills, &old_name, new_name.clone(), new_amount) {
        Ok(()) => println!("  [*] Updated -> '{}' - ${:.2}\n", new_name, new_amount),
        Err(message) => println!("  [!] {message}\n"),
    }
}

fn print_menu() {
    println!("========================================");
    println!(" Bill Manager");
    println!("========================================");
    println!(" 1. Add bill");
    println!(" 2. View bills");
    println!(" 3. Remove bill");
    println!(" 4. Edit bill");
    println!(" q. Quit");
}

fn main() {
    let mut bills: HashMap<String, f64> = HashMap::new();

    println!("\nWelcome to the Bill Manager!\n");
    loop {
        print_menu();
        let choice = get_input(" Your choice: ");
        match choice.to_lowercase().as_str() {
            "1" | "add" => add_bill(&mut bills),
            "2" | "view" => view_bills(&bills),
            "3" | "remove" => remove_bill(&mut bills),
            "4" | "edit" => edit_bill(&mut bills),
            "q" | "quit" => {
                println!("\nGoodbye!\n");
                break;
            }
            "" => {}
            _ => println!("  [!] Unknown option '{}'.\n", choice),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed() -> HashMap<String, f64> {
        let mut bills = HashMap::new();
        bills.insert("Rent".to_string(), 1200.0);
        bills.insert("Power".to_string(), 90.0);
        bills.insert("Water".to_string(), 40.0);
        bills
    }

    #[test]
    fn stage1_add_and_view_data() {
        let mut bills = HashMap::new();
        assert!(add_bill_data(&mut bills, "Gym".to_string(), 45.5).is_ok());
        assert_eq!(bills.get("Gym"), Some(&45.5));
    }

    #[test]
    fn stage1_duplicate_rejected() {
        let mut bills = seed();
        let result = add_bill_data(&mut bills, "Rent".to_string(), 1000.0);
        assert!(result.is_err());
        assert_eq!(bills.get("Rent"), Some(&1200.0));
    }

    #[test]
    fn stage2_remove_bill() {
        let mut bills = seed();
        assert_eq!(remove_bill_data(&mut bills, "Water"), Some(40.0));
        assert!(!bills.contains_key("Water"));
    }

    #[test]
    fn stage2_remove_nonexistent() {
        let mut bills = seed();
        assert_eq!(remove_bill_data(&mut bills, "Phone"), None);
        assert_eq!(bills.len(), 3);
    }

    #[test]
    fn stage3_edit_name_and_amount() {
        let mut bills = seed();
        let result = edit_bill_data(&mut bills, "Power", "Electricity".to_string(), 95.0);
        assert!(result.is_ok());
        assert_eq!(bills.get("Electricity"), Some(&95.0));
        assert!(!bills.contains_key("Power"));
    }

    #[test]
    fn stage3_edit_reject_rename_over_existing() {
        let mut bills = seed();
        let result = edit_bill_data(&mut bills, "Water", "Rent".to_string(), 40.0);
        assert!(result.is_err());
        assert_eq!(bills.get("Water"), Some(&40.0));
    }
}
