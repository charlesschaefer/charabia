use std::borrow::Cow;

// Import `Normalizer` trait.
use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::{is_nfkd_quick, IsNormalized};

// Make a small documentation of the specialized Normalizer like below.
/// Implements an unicode compatibility decomposition () [`Normalizer`] for Unicode.
///
/// This Normalizer uses [`unicode_normalization`] internally to normalize the provided token.
pub struct CompatibilityDecompositionNormalizer;

impl Normalizer for CompatibilityDecompositionNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        _options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        let new_lemma = match is_nfkd_quick(token.lemma().chars()) {
            IsNormalized::Yes => String::from(token.lemma()),
            IsNormalized::No => token.lemma().chars().nfkd().collect::<String>(),
            IsNormalized::Maybe => token.lemma().chars().nfkd().collect::<String>(),
        };

        token.lemma = Cow::Owned(new_lemma);

        // Create an iterator over the normalized token.
        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        // apply the normalization only on non-ascii chars.
        true
    }
}

// Include the newly implemented Normalizer in the tokenization pipeline:
//     - change the name of the file `dummy_example.rs` to `dummy.rs`
//     - import module by adding `mod dummy;` (filename) in `normalizer/mod.rs`
//     - Add Normalizer in `NORMALIZERS` in `normalizer/mod.rs`
//     - check if it didn't break any test or benhchmark

// Test the normalizer:
#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("PascalCase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ПаскальКейс".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                // lowercased
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // lowercased
                lemma: Owned("паскалькейс".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("paskal'keis".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(
        CompatibilityDecompositionNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}

// Your Normalizer will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
