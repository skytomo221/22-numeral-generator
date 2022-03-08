use crate::{
    phoneme::Phoneme,
    recipe::{SuperLanguage, SuperWord},
};
use core::fmt;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub first_consonant: Phoneme,
    pub vowel: Phoneme,
    pub second_consonant: Phoneme,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}",
            self.first_consonant, self.vowel, self.second_consonant
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct CandidateNumber {
    pub score: f64,
    pub number: Number,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct CandidateNumbers {
    pub score: f64,
    pub numbers: Vec<CandidateNumber>,
}

pub struct NumberGenerator {
    pub super_languages: Vec<SuperLanguage>,
    pub super_words: Vec<SuperWord>,
    pub words: Vec<CandidateNumbers>,
    weight_sum: f64,
    regular_weights: HashMap<String, f64>,
    candidate_phonemes: HashMap<usize, HashMap<Phoneme, f64>>,
}

impl NumberGenerator {
    fn initialize_weight_sum(&mut self) {
        self.weight_sum = self
            .super_languages
            .iter()
            .map(|super_language| super_language.population)
            .sum()
    }

    fn get_super_language(&self, language: &str) -> &SuperLanguage {
        self.super_languages
            .iter()
            .find(|super_language| super_language.language == language)
            .unwrap()
    }

    pub fn get_population(&self, language: &str) -> f64 {
        self.get_super_language(language).population
    }

    fn get_regular_weight(&self, language: &str) -> f64 {
        self.get_population(language) / self.weight_sum
    }

    fn initialize_regular_weights(&mut self) {
        self.regular_weights = self
            .super_languages
            .iter()
            .map(|super_language| {
                (
                    super_language.language.clone(),
                    self.get_regular_weight(super_language.language.as_str()),
                )
            })
            .collect();
    }

    fn initialize_candidate_phonemes(&mut self) {
        self.candidate_phonemes = HashMap::new();
        for super_word in &self.super_words {
            let number = super_word.meaning.parse::<usize>().unwrap();
            let mut v = HashMap::new();
            for origin in &super_word.origins {
                let loan: HashSet<&Phoneme> = origin.loan.as_ref().unwrap().into_iter().collect();
                for phoneme in loan {
                    *v.entry(phoneme.clone()).or_insert(0.0) +=
                        self.regular_weights[&origin.language];
                }
            }
            self.candidate_phonemes.insert(number, v);
        }
    }

    fn initialize(&mut self) {
        self.initialize_weight_sum();
        self.initialize_regular_weights();
        self.initialize_candidate_phonemes();
    }

    pub fn new(
        super_languages: Vec<SuperLanguage>,
        super_words: Vec<SuperWord>,
    ) -> NumberGenerator {
        let mut number_generator = NumberGenerator {
            super_languages,
            super_words,
            words: Vec::new(),
            weight_sum: 0.0,
            regular_weights: HashMap::new(),
            candidate_phonemes: HashMap::new(),
        };
        number_generator.initialize();
        number_generator
    }

    pub fn generate(&mut self) {
        let consonants = vec![
            Phoneme::P,
            Phoneme::B,
            Phoneme::T,
            Phoneme::D,
            Phoneme::K,
            Phoneme::G,
            Phoneme::M,
            Phoneme::N,
            Phoneme::R,
            Phoneme::F,
            Phoneme::V,
            Phoneme::S,
            Phoneme::Z,
            Phoneme::C,
            Phoneme::J,
            Phoneme::X,
            Phoneme::H,
            Phoneme::L,
            Phoneme::Y,
            Phoneme::W,
        ];
        let vowels = vec![Phoneme::A, Phoneme::E, Phoneme::I, Phoneme::O, Phoneme::U];
        for all_consonants in consonants.iter().permutations(10).permutations(2) {
            let first_consonants = &all_consonants[0];
            let second_consonants = &all_consonants[1];
            let raw_candiate_numbers = first_consonants.iter().zip(second_consonants.iter());
            let mut candiate_numbers = CandidateNumbers {
                score: 0.0,
                numbers: Vec::<CandidateNumber>::new(),
            };
            for (index, candidate_number) in raw_candiate_numbers.enumerate() {
                let first_consonant = **candidate_number.0;
                let second_consonant = **candidate_number.1;
                let vowel = vowels[index % vowels.len()];
                let number = Number {
                    first_consonant,
                    vowel,
                    second_consonant,
                };
                let candiate_number = CandidateNumber {
                    score: self.number_score(index, &number),
                    number,
                };
                candiate_numbers.numbers.push(candiate_number);
            }
            candiate_numbers.score = self.candidate_numbers_score(&candiate_numbers.numbers);
            println!(
                "{} | {}",
                &candiate_numbers
                    .numbers
                    .iter()
                    .map(|c| { format!("{}", c.number) })
                    .join(", "),
                &candiate_numbers.score
            );
            self.words.push(candiate_numbers);
        }
    }

    fn number_score(&self, index: usize, number: &Number) -> f64 {
        let mut score = 0.0;
        if self.candidate_phonemes[&index].contains_key(&number.first_consonant) {
            score += self.candidate_phonemes[&index][&number.first_consonant];
        }
        if self.candidate_phonemes[&index].contains_key(&number.second_consonant) {
            score += self.candidate_phonemes[&index][&number.second_consonant];
        }
        score
    }

    fn candidate_numbers_score(&self, candidate_numbers: &Vec<CandidateNumber>) -> f64 {
        candidate_numbers
            .iter()
            .map(|candidate_number| candidate_number.score)
            .sum()
    }
}
