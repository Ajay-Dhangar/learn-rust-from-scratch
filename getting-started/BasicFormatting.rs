// ===========================================
// Example 1: Basic Formatting
// ===========================================

fn main() {
    println!("{} is a {} company", "Educative", "Software"); // Educative is a Software company
}

// ===========================================
// Example 2: Positional Arguments
// ===========================================

fn main() {
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
}

// ===========================================
// Example 3: Formatting Numbers
// ===========================================

fn main() {
    println!("{}{}", 2, 1);
}