fn main() {
    let state_code = "MH";
    let state = match state_code {
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadake",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}", state);
}