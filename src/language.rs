use core::fmt;

use serde::{
    de::Visitor,
    ser::{Serialize, Serializer},
    Deserialize, Deserializer,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Language {
    Chinese,
    English,
    Spanish,
    Hindi,
    Portuguese,
    Russian,
    French,
    Arabic,
    Japanese,
    Indonesian,
    Urdu,
    German,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Chinese => "zh",
                Language::English => "en",
                Language::Spanish => "es",
                Language::Hindi => "hi",
                Language::Portuguese => "pt",
                Language::Russian => "ru",
                Language::French => "fr",
                Language::Arabic => "ar",
                Language::Japanese => "ja",
                Language::Indonesian => "id",
                Language::Urdu => "ur",
                Language::German => "de",
            }
        )
    }
}

pub trait StringExt {
    fn iso_639(&self) -> Option<Language>;
}

impl StringExt for String {
    fn iso_639(&self) -> Option<Language> {
        match self.as_str() {
            "zh" => Some(Language::Chinese),
            "en" => Some(Language::English),
            "es" => Some(Language::Spanish),
            "hi" => Some(Language::Hindi),
            "pt" => Some(Language::Portuguese),
            "ru" => Some(Language::Russian),
            "fr" => Some(Language::French),
            "ar" => Some(Language::Arabic),
            "ja" => Some(Language::Japanese),
            "id" => Some(Language::Indonesian),
            "ur" => Some(Language::Urdu),
            "de" => Some(Language::German),
            _ => None,
        }
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PhonemeVisitor;

impl<'de> Visitor<'de> for PhonemeVisitor {
    type Value = Language;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a phoneme")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.to_owned()
            .iso_639()
            .ok_or(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"a Phoneme",
            ))
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Language, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PhonemeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_test() {
        assert_eq!(Language::Chinese.to_string(), "zh");
        assert_eq!("zh".to_string().iso_639().unwrap(), Language::Chinese);
    }

    use crate::convert::{ipa_to_phonemes, phonemes_to_loan};
    #[test]
    fn test_language_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??l??????w??d????")), "langwidj");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??l?????.??wa")), "lengwa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("b??????.??????")), "baca");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("l??.??w??")), "ligwa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("(j)????z??k")), "yizik");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("l??????")), "lag");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("???????????o??")), "gengo");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ba??hasa")), "bahasa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("z??.b????n")), "zuban");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("????p??a??x??")), "cprax-");
    }

    #[test]
    fn test_cat_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("m??o")), "mao");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("k??t")), "kat");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("????a.t??o")), "gato");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("b??l??.l??i??")), "billi");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("bi???al")), "biral");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??????.t????")), "gatu");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??ko??k??")), "kock-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??a")), "ca");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("qi??????")), "kitt");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ne??ko??")), "neko");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("kut????????")), "kutcin");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("bill??")), "billi");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??kats??")), "kats-");
    }

    #[test]
    fn test_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("??p??a??x??")), "cprax-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ko??k??")), "kock-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("qit??t??")), "kitt");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("jab????k??")), "yabl-k-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("m????????n")), "meg-n");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("kn??i????")), "knig-");
    }
}
