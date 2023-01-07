/// Converts integers to human-readable integers separated by
/// commas, e.g. "1000000" displays as "1,000,000" when fed through
/// this function.
pub fn format_int(int: u64) -> String {
    let mut string = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}