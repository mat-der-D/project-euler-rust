// <p>Using <a href="resources/documents/0022_names.txt">names.txt</a> (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.</p>
// <p>For example, when the list is sorted into alphabetical order, COLIN, which is worth $3 + 15 + 12 + 9 + 14 = 53$, is the $938$th name in the list. So, COLIN would obtain a score of $938 \times 53 = 49714$.</p>
// <p>What is the total of all the name scores in the file?</p>

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

fn char_to_value(c: char) -> u32 {
    let raw: u32 = c.into();
    raw - 64
}

fn evaluate_name(name: &str) -> u32 {
    name.chars().map(|c| char_to_value(c)).sum()
}

fn load_names(path: &Path) -> Result<Vec<String>, anyhow::Error> {
    let fs = File::open(path)?;
    let mut reader = BufReader::new(fs);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let names: Vec<String> = content.split(",").map(|s| s.replace("\"", "")).collect();
    Ok(names)
}

fn main() -> Result<(), anyhow::Error> {
    let path = Path::new("src/0022_names.txt");
    let mut names = load_names(&path)?;
    names.sort();
    let total_score: u32 = names
        .into_iter()
        .enumerate()
        .map(|(i, name)| (i as u32 + 1) * evaluate_name(&name))
        .sum();
    println!("{total_score}");
    Ok(())
}
