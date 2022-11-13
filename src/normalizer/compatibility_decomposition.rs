// Import `Normalizer` trait.
use super::{CharNormalizer, CharOrStr};
use crate::{Token, Script};

use unicode_normalization::{is_nfkd, is_nfkd_quick, IsNormalized, UnicodeNormalization};

// Make a small documentation of the specialized Normalizer like below.
/// Implements an unicode compatibility decomposition () [`Normalizer`] for Unicode.
///
/// This Normalizer uses [`unicode_normalization`] internally to normalize the provided token.
pub struct CompatibilityDecompositionNormalizer;

impl CharNormalizer for CompatibilityDecompositionNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        let box_char = Box::new(Some(c).into_iter());
        let normalized = match is_nfkd_quick(box_char) {
            IsNormalized::Yes => String::from(c),
            IsNormalized::No => c.nfkd().collect::<String>(),
            IsNormalized::Maybe => String::from(c), //c.nfkd().collect::<String>(),
        };

        Some(normalized.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        
        // apply the normalization only when needed
        // @TODO: maybe we need to apply the composition on other scripts/languages too?
        matches!(token.script, Script::Latin | Script::Cyrillic | Script::Greek | Script::Georgian)
            && !token.lemma().is_ascii()
            && !is_nfkd(token.lemma())
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
    use crate::normalizer::{Normalizer, NormalizerOption};

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                // 1234
                lemma: Owned("①②③④".to_string()),
                char_end: 4,
                byte_end: 12,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // Both \u212B (Å) and \u00C5 (Å) should be decomposed to
                // A+\030A (Å)
                lemma: Owned("Å Å Å".to_string()),
                char_end: 6,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // decomposes é (\u00e9) to é (e + \u0301)
                lemma: Owned("élégant".to_string()),
                char_end: 7,
                byte_end: 9,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // decomposes ﬁ (\ufb01) to fi (\u0066 + \u0069)
                lemma: Owned("ﬁ".to_string()),
                char_end: 1,
                byte_end: 3,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                // 1234
                lemma: Owned("1234".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // Both \u212B (Å) and \u00C5 (Å) should be decomposed to
                // A+\030A (Å)
                lemma: Owned("Å Å Å".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // decomposes "é" (\u00e9) to "é" (e + \u0301)
                lemma: Owned("élégant".to_string()),
                char_end: 9,
                byte_end: 11,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // decomposes ﬁ (\ufb01) to fi (\u0066 + \u0069)
                lemma: Owned("fi".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pipeline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("1234".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("a a a".to_string()),
                char_end: 5,
                byte_end: 5,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("elegant".to_string()),
                char_end: 7,
                byte_end: 7,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("fi".to_string()),
                char_end: 2,
                byte_end: 2,
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
