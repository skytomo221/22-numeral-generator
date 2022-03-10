use crate::{
    phoneme::Phoneme,
    recipe::{SuperLanguage, SuperWord},
};
use core::fmt;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Number {
    pub first_consonant: Phoneme,
    pub vowel: Phoneme,
    pub second_consonant: Phoneme,
}

impl Number {
    fn duplicate_first_consonant(&self, other: Number) -> bool {
        self.first_consonant == other.first_consonant
    }

    fn duplicate_second_consonant(&self, other: Number) -> bool {
        self.second_consonant == other.second_consonant
    }
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

#[derive(Debug)]
struct NumberIterator {
    candidate_consonants: Vec<Phoneme>,
    vowel: Phoneme,
    first_consonant_index: usize,
    second_consonant_index: usize,
}

impl NumberIterator {
    pub fn new(candidate_consonants: Vec<Phoneme>, vowel: Phoneme) -> NumberIterator {
        NumberIterator {
            candidate_consonants,
            vowel,
            first_consonant_index: 0,
            second_consonant_index: 0,
        }
    }

    fn end(&self) -> bool {
        self.first_consonant_index >= self.candidate_consonants.len()
    }

    pub fn carry_up_index(&mut self) {
        self.second_consonant_index = 0;
        self.first_consonant_index += 1;
    }

    fn next_index(&mut self) {
        if self.end() {
            return;
        } else if self.second_consonant_index + 1 >= self.candidate_consonants.len() {
            self.carry_up_index();
        } else {
            self.second_consonant_index += 1;
        }
    }

    pub fn reload(&mut self) {
        self.first_consonant_index = 0;
        self.second_consonant_index = 0;
    }
}

impl Iterator for NumberIterator {
    type Item = Number;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end() {
            None
        } else {
            while self.first_consonant_index == self.second_consonant_index {
                self.next_index();
                if self.end() {
                    return None;
                }
            }
            let first_consonant = self.candidate_consonants[self.first_consonant_index];
            let second_consonant = self.candidate_consonants[self.second_consonant_index];
            let number = Some(Number {
                first_consonant,
                vowel: self.vowel,
                second_consonant,
            });
            self.next_index();
            number
        }
    }
}

struct NumbersIterator {
    end: bool,
    number_itrators: Vec<NumberIterator>,
    numbers: Vec<Number>,
}

impl NumbersIterator {
    pub fn new(candidates: Vec<Vec<Phoneme>>) -> NumbersIterator {
        let vowels = vec![Phoneme::A, Phoneme::E, Phoneme::I, Phoneme::O, Phoneme::U];
        let mut number_itrators = Vec::new();
        for (index, candidate_consonants) in candidates.iter().enumerate() {
            number_itrators.push(NumberIterator::new(
                candidate_consonants.clone(),
                vowels[index % vowels.len()],
            ));
        }
        let mut numbers = Vec::new();
        for number_itrator in &mut number_itrators {
            numbers.push(number_itrator.next().unwrap());
        }
        NumbersIterator {
            end: false,
            number_itrators,
            numbers,
        }
    }

    fn raw_next(&mut self, index: usize) -> Option<Vec<Number>> {
        if self.end {
            None
        } else if let Some(number) = self.number_itrators[index].next() {
            self.numbers[index] = number;
            Some(self.numbers.clone())
        } else if index == 0 {
            self.end = true;
            None
        } else {
            self.number_itrators[index].reload();
            self.numbers[index] = self.number_itrators[index].next().unwrap();
            self.raw_next(index - 1)
        }
    }

    fn raw_next_and_get_index(&mut self, index: usize) -> usize {
        if self.end {
            0
        } else if let Some(number) = self.number_itrators[index].next() {
            self.numbers[index] = number;
            index
        } else if index == 0 {
            self.end = true;
            0
        } else {
            self.number_itrators[index].reload();
            self.numbers[index] = self.number_itrators[index].next().unwrap();
            self.raw_next_and_get_index(index - 1)
        }
    }

    fn avoid_duplicate(&mut self) -> bool {
        let mut duplicate = false;
        let mut index = 1;
        while index < self.numbers.len() {
            let number = self.numbers[index];
            if self.end {
                return false;
            } else if self
                .numbers
                .iter()
                .take(index)
                .any(|head| head.duplicate_first_consonant(number))
            {
                duplicate = true;
                self.number_itrators[index].carry_up_index();
                if index + 1 < self.numbers.len() {
                    for index in (index + 1)..self.numbers.len() {
                        self.number_itrators[index].reload();
                        self.raw_next(index);
                    }
                }
                index = self.raw_next_and_get_index(index);
            } else if self
                .numbers
                .iter()
                .take(index)
                .any(|head| head.duplicate_second_consonant(number))
            {
                duplicate = true;
                if index + 1 < self.numbers.len() {
                    for index in (index + 1)..self.numbers.len() {
                        self.number_itrators[index].reload();
                        self.raw_next(index);
                    }
                }
                index = self.raw_next_and_get_index(index);
            } else {
                index += 1;
            }
        }
        duplicate
    }
}

impl Iterator for NumbersIterator {
    type Item = Vec<Number>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.avoid_duplicate() {
            Some(self.numbers.clone())
        } else {
            self.raw_next(9)
        }
    }
}

pub struct NumberGenerator {
    pub super_languages: Vec<SuperLanguage>,
    pub super_words: Vec<SuperWord>,
    pub words: Vec<CandidateNumbers>,
    weight_sum: f64,
    regular_weights: HashMap<String, f64>,
    candidate_phonemes: Vec<HashMap<Phoneme, f64>>,
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
        self.candidate_phonemes = Vec::new();
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
            candidate_phonemes: Vec::new(),
        };
        number_generator.initialize();
        number_generator
    }

    pub fn generate(&mut self) {
        let vowels = vec![Phoneme::A, Phoneme::E, Phoneme::I, Phoneme::O, Phoneme::U];
        let candidates = self
            .candidate_phonemes
            .iter()
            .map(|scores| {
                let mut keys = scores
                    .keys()
                    .cloned()
                    .filter(|p| !vowels.contains(p))
                    .collect::<Vec<_>>();
                keys.sort();
                keys.dedup();
                keys
            })
            .collect();
        let numbers_iterator = NumbersIterator::new(candidates);
        let mut max_score = 0.0;
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
        println!("");
        println!(
            "| Consonant |    0      1      2      3      4      5      6      7      8      9   |"
        );
        println!(
            "|:---------:|:----------------------------------------------------------------------|"
        );
        for consonant in consonants {
            print!("|         {:?} |", consonant);
            self.candidate_phonemes
                .iter()
                .map(|candidate_phoneme| {
                    if candidate_phoneme.contains_key(&consonant) {
                        candidate_phoneme[&consonant]
                    } else {
                        0.0
                    }
                })
                .for_each(|x| print!(" {:.4}", x));
            println!(" |");
        }
        println!("");
        println!("|      Line |  0   1   2   3   4   5   6   7   8   9  |    0      1      2      3      4      5      6      7      8      9   |  Total |");
        println!("|:---------:|:---------------------------------------:|:---------------------------------------------------------------------:|:------:|");
        for (index, consonants) in numbers_iterator.enumerate() {
            let mut candiate_numbers = CandidateNumbers {
                score: 0.0,
                numbers: Vec::<CandidateNumber>::new(),
            };
            for (index, &number) in consonants.iter().enumerate() {
                let candiate_number = CandidateNumber {
                    score: self.number_score(index, &number),
                    number,
                };
                candiate_numbers.numbers.push(candiate_number);
            }
            candiate_numbers.score = self.candidate_numbers_score(&candiate_numbers.numbers);
            if candiate_numbers.score >= max_score {
                max_score = candiate_numbers.score;
                println!(
                    "|{:10} | {} | {} | {:.4} |",
                    index,
                    &candiate_numbers
                        .numbers
                        .iter()
                        .map(|c| { format!("{}", c.number) })
                        .join(" "),
                    &candiate_numbers
                        .numbers
                        .iter()
                        .map(|c| { format!("{:.4}", c.score) })
                        .join(" "),
                    &candiate_numbers.score
                );
                self.words.push(candiate_numbers);
            }
        }
    }

    fn number_score(&self, index: usize, number: &Number) -> f64 {
        let mut score = 0.0;
        if self.candidate_phonemes[index].contains_key(&number.first_consonant) {
            score += self.candidate_phonemes[index][&number.first_consonant];
        }
        if self.candidate_phonemes[index].contains_key(&number.second_consonant) {
            score += self.candidate_phonemes[index][&number.second_consonant];
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
