use tabulate::{Headers, TabulateOptions, tabulate};

fn main() {
    let planets = vec![
        vec!["Planet", "Radius (km)", "Mass (10^24 kg)"],
        vec!["Mercury", "2440", "0.330"],
        vec!["Venus", "6052", "4.87"],
        vec!["Earth", "6371", "5.97"],
        vec!["Mars", "3390", "0.642"],
    ];

    let table = tabulate(
        planets,
        TabulateOptions::new()
            .headers(Headers::FirstRow)
            .table_format("grid"),
    )
    .expect("tabulation succeeds");

    println!("{table}");
}
