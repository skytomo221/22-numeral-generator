use bacitit_word_generator::convert;
use bacitit_word_generator::number_generator::NumberGenerator;
use bacitit_word_generator::phoneme::Phoneme;
use bacitit_word_generator::recipe::Recipe;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::vec;
use std::{fs, io::Write};

/*
fn export_word(candidate_words: &WordGenerator) {
    let best_word = &candidate_words.words[0];
    let mut output = format!(
        "# {}\n\n## Meaning\n\n{}",
        convert::phonemes_to_loan(&best_word.word),
        candidate_words.super_word.meaning
    );
    let candidates_info = {
        let mut s = "|Word|Score|\n|:-:|:-:|\n".to_string();
        let b = candidate_words.words.iter().take(10);
        for c in b {
            println!("{:?}", c);
            s.push_str(&format!(
                "|{}|{:.6}|\n",
                convert::phonemes_to_loan(&c.word),
                c.score
            ));
        }
        s
    };
    let langs_info = {
        let mut s = "|ISO 639-1|Weight|Regular weight|Origin word|IPA|Loanword|\n|:-:|:-:|:-:|:-:|:-:|:-:|\n"
            .to_string();
        for origin in &candidate_words.super_word.origins {
            let language = &origin.language;
            s.push_str(&format!(
                "|{}|{}|{:.4}|{}|{}|{}|\n",
                language,
                candidate_words.get_population(&language),
                candidate_words.get_population(&language) / candidate_words.weight_sum,
                origin.word,
                origin.ipa.as_ref().unwrap(),
                convert::phonemes_to_loan(origin.loan.as_ref().unwrap()),
            ));
        }
        s
    };
    output.push_str(&format!(
        "\n\n## Candidates\n\n{}\n## Origins\n\nWeight sum: {}\n{}",
        candidates_info, candidate_words.weight_sum, langs_info
    ));
    let mut f = fs::File::create(format!(
        "./export/dic/{}.md",
        convert::phonemes_to_loan(&best_word.word)
    ))
    .unwrap();
    f.write_all(output.as_bytes()).unwrap();
}

fn export_word_list(generated: &BTreeMap<String, Vec<Phoneme>>) {
    let mut f = fs::File::create("./export/word-list.md").unwrap();
    let output = {
        let mut s = "# Word List\n\n|Spell|Meaning|\n|:-:|:-:|\n".to_string();
        for x in generated {
            s.push_str(&format!(
                "|[{0}](./dic/{0}.md)|{1}|\n",
                convert::phonemes_to_loan(x.1),
                x.0
            ));
        }
        s
    };
    f.write_all(output.as_bytes()).unwrap();
}

fn export_result(recipe: Recipe) {
    serde_json::to_writer_pretty(&File::create("./data/result.json").unwrap(), &recipe).unwrap();
}
*/

pub fn main() {
    let recipe_file = File::open("data/recipe.json").unwrap();
    let recipe_reader = BufReader::new(recipe_file);
    let recipe: Recipe = serde_json::from_reader(recipe_reader).unwrap();
    let recipe = recipe.complement();
    //let mut generated = BTreeMap::new();
    println!("super_words.words.len() = {}", recipe.super_words.len());
    let mut number_generator =
        NumberGenerator::new(recipe.super_languages.clone(), recipe.super_words.clone());
    number_generator.generate();
    /*
    export_word(&number_generator);
    generated.insert(
        number_generator.super_word.meaning.clone(),
        (&number_generator.words[0].word).clone(),
    );
    export_result(recipe.clone());
    export_word_list(&generated);
    */
}
