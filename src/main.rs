use bacitit_word_generator::number_generator::NumberGenerator;
use bacitit_word_generator::recipe::Recipe;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    let recipe_file = File::open("data/recipe.json").unwrap();
    let recipe_reader = BufReader::new(recipe_file);
    let recipe: Recipe = serde_json::from_reader(recipe_reader).unwrap();
    let recipe = recipe.complement();
    println!("Ready...");
    let mut number_generator =
        NumberGenerator::new(recipe.super_languages.clone(), recipe.super_words.clone());
    number_generator.generate();
}
