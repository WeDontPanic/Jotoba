#![allow(dead_code)]

pub mod inflection;
pub mod owned_morpheme;
pub mod part;

use crate::grammar;
use igo_unidic::{Morpheme, WordClass};

pub trait FromMorphemes<'a, 'b>: Sized {
    fn from(parts: Vec<Morpheme<'a, 'b>>, pos: usize) -> Option<Self>;
}

impl<'b> FromMorphemes<'static, 'b> for (Vec<&'static str>, usize) {
    #[inline]
    fn from(parts: Vec<Morpheme<'static, 'b>>, pos: usize) -> Option<Self> {
        let parts = parts.iter().map(|i| i.lexeme).collect::<Vec<_>>();
        if parts.is_empty() {
            return None;
        }
        Some((parts, pos))
    }
}

/// An analyzer for sentences/text to portion morphemes together based on rules
pub struct SentenceAnalyzer<'input> {
    grammar: &'input grammar::Analyzer,
    morphemes: Vec<Morpheme<'static, 'input>>,
}

impl<'input> SentenceAnalyzer<'input> {
    /// Create a new SentenceAnalyer
    pub fn new(
        grammar: &'input grammar::Analyzer,
        morphemes: Vec<Morpheme<'static, 'input>>,
    ) -> Self {
        Self { grammar, morphemes }
    }

    /// Returns `true` if SentenceAnalyer would yield no words
    pub fn is_empty(&self) -> bool {
        self.morphemes.is_empty()
    }

    /// Executes the analyzation and returns a set of Words which are built out of 1..n morphemes
    pub fn analyze<O: FromMorphemes<'static, 'input>>(&self) -> Vec<O> {
        let morphs = &self.morphemes;

        let mut out = Vec::new();
        let mut pos = 0;

        loop {
            let curr = match morphs.get(pos) {
                Some(n) => n,
                None => break,
            };

            // Collect rules of next n morphemes
            let rules: Vec<_> = morphs[pos..]
                .iter()
                .enumerate()
                .map(|(pos, m)| map_morph_to_rule(pos, m))
                // if a morphemes does not have a rule, we can stop
                // collecting all rules since the analyzer would stop
                // at a `None` rule anyways
                .take_while(|i| i.is_some())
                .map(|i| i.unwrap())
                .collect();

            let n_matching = self.grammar.check(&rules);
            let mut parts = (0..n_matching).map(|i| morphs[pos + i]).collect::<Vec<_>>();

            if parts.is_empty() {
                parts.push(*curr);
                pos += 1;
            }

            pos += n_matching;

            let word_position = out.len();
            if let Some(word) = O::from(parts, word_position) {
                out.push(word);
            }
        }

        out
    }

    /// Returns the raw morphemes of the sentence
    pub fn morphemes(&self) -> &Vec<Morpheme<'_, '_>> {
        &self.morphemes
    }

    pub fn debug(&self) {
        for i in self.morphemes.iter() {
            println!("{}\t({})({:?})", i.surface, i.lexeme, i.word_class);
        }

        println!();

        for i in self.analyze::<part::Part>() {
            print!(
                "{}|",
                i.morphemes()
                    .iter()
                    .map(|i| i.surface.as_str())
                    .collect::<String>()
            );
        }
        println!();
    }
}

pub(crate) fn map_morph_to_rule(pos: usize, morph: &Morpheme<'_, '_>) -> Option<&'static str> {
    if morph.surface == "じゃ" {
        return Some("じゃ");
    }

    if morph.lexeme == "ない" {
        return Some("ない");
    }

    if morph.lexeme == "たい" {
        return Some("たい");
    }

    if morph.lexeme == "た" || morph.lexeme == "だ" {
        return Some("た");
    }

    if morph.lexeme == "たり" || morph.lexeme == "だり" {
        return Some("たり");
    }

    if morph.lexeme == "てる" || morph.lexeme == "でる" {
        return Some("てる");
    }

    if morph.lexeme == "て" || morph.lexeme == "で" {
        return Some("て");
    }

    if morph.lexeme == "ある" {
        return Some("ある");
    }

    if morph.lexeme == "いる" {
        return Some("いる");
    }

    if morph.lexeme == "ます" {
        return Some("ます");
    }

    if morph.lexeme == "られる" {
        return Some("られる");
    }

    if morph.lexeme == "れる" {
        return Some("れる");
    }

    if morph.lexeme == "しまう" {
        return Some("しまう");
    }

    if morph.lexeme == "ちゃう" || morph.lexeme == "じゃう" {
        return Some("ちゃう");
    }

    if morph.lexeme == "おく" {
        return Some("おく");
    }

    if morph.lexeme == "とく" || morph.lexeme == "どく" {
        return Some("とく");
    }

    if morph.lexeme == "ば" {
        return Some("ば");
    }

    if morph.lexeme == "ぬ" {
        return Some("ん");
    }

    if morph.lexeme == "です" {
        return Some("です");
    }

    if morph.surface == "さ" && morph.lexeme == "する" {
        return Some("さ");
    }

    if morph.lexeme == "させる" {
        return Some("させる");
    }

    if morph.lexeme == "せる" {
        return Some("せる");
    }

    if morph.lexeme == "頂" && morph.surface == "頂" && morph.reading == "イタダキ" {
        return Some("いただき");
    }

    // てみる form. Can only be applied if not pos==0. If pos == 0, the word 見る is being used
    // which does not go with the みる rule
    if (morph.surface == "み" || morph.lexeme == "みる") && pos > 0 {
        return Some("てみる");
    }

    if let WordClass::Noun(noun_type) = morph.word_class {
        return Some(match noun_type {
            igo_unidic::NounType::Numeral => "NR",
            _ => "N",
        });
    }

    if morph.word_class.is_adjective() {
        return Some("AD");
    }

    if morph.word_class.is_verb() {
        return Some("V");
    }

    None
}

/*
 * TODO: fix Parser not being static
#[cfg(test)]
mod test {
    use crate::grammar::Analyzer;
    use igo_unidic::Parser;

    use super::*;

    fn get_parser() -> Parser {
        igo_unidic::Parser::new("../../unidic-mecab").unwrap()
    }

    fn get_g_analyzer() -> &'static Analyzer {
        crate::analyzer::get_grammar_analyzer()
    }

    #[test]
    pub fn test_analyzer() {
        let analyzer = get_g_analyzer();
        assert!(analyzer.rules().check());
        assert_eq!(analyzer.check(&["ない", "て"]), 2);
        assert_eq!(analyzer.check(&["ない", "abc"]), 1);
        assert_eq!(analyzer.check(&["い", "い"]), 0);
        assert_eq!(analyzer.check(&["V", "たい", "ない"]), 3);
    }

    #[test]
    pub fn test_single_words() {
        let words = &[
            "見たくない",
            "見る",
            "見ます",
            "見たい",
            "見たくない",
            "見たくなくて",
            "見たくなかった",
            "見て",
            "見ている",
            "見ています",
            "見てある",
            "見てあります",
            "見ない",
            "見なくて",
            "見なかった",
            "見ません",
            "見ませんでした",
            "見られる",
            "見られて",
            "見られている",
            "見られない",
            "見られなくて",
            "見られなかった",
            "見ちゃう",
            "見てしまう",
            "持っていない",
            "美味しい",
            "美味しかった",
            "美味しくない",
            "美味しくなくて",
            "美味しくなかった",
            "美味しくて",
            "便利",
            "じゃない",
            "じゃなかった",
            "じゃなくて",
            "いちゃう",
            "いてしまう",
            "行ってしまう",
            "行っちゃう",
        ];

        let analyzer = get_g_analyzer();
        let parser = get_parser();

        for word in words {
            let sentenec_parser = SentenceAnalyer::new(&analyzer, parser.parse(word));
            let analyzed = sentenec_parser.analyze();
            if analyzed.len() != 1 {
                println!("{word}");
                panic!("Word split to much");
            }
            if analyzed[0].get_inflected() != *word {
                println!("{word} != {}", analyzed[0].get_inflected());
                panic!("word is not equal to surface");
            }
        }
    }

    #[test]
    pub fn test_long_texts() {
        let analyzer = get_g_analyzer();
        let parser = get_parser();

        let inp = &[
            "１８日午後０時５５分頃、札幌市中央区の２２階建てホテルの１４階にある屋外スペースで、女優の神田沙也加さん（３５）が意識不明の状態で倒れているのが見つかり、約９時間後に搬送先の病院で死亡した",
            "北海道警は、宿泊していた高層階の部屋の窓から転落した可能性があるとみている。ホテル関係者によると、窓は縦、横とも約１メートル。全開できないよう安全装置が取り付けられていたという",
            "神田沙也加さん死亡、ホテル高層階の部屋から転落か…連絡つかず事務所が警察に通報",
            "昨今「ウケる」は、面白いという意味で頻繁に使用されています。如何なる面白さにも用いることができ、「この芸人さん超ウケるよね」とか「この遊びウケる」、「この蛇の動き超ウケる」というように使われます。",
            "しかし、「ウケる」の定義の幅が多いため、「君の顔ウケるよね～」なんて言うと、言った本人に悪気がなくても、言われた側は気に障ってしまうかもしれません。「ウケる」という言葉は便利ですが、時と場所、相手を選んで使うようにしましょう。",
            "「ウケる」という単語には「超ウケる」というような表現も存在します。これは「超面白い」と同様に「ウケる」に「超」が付いただけのものでありますが、「ウケる」の意味を強調して、本当に面白いさまを表します。",
                "また「大ウケ」という言葉もあります。こちらは主観的に用いられがちな「ウケる」と違い、客観的な評価を表してしばしば使用されます。たとえば「二次会で披露したギャグが大ウケだった」という場合、自身の芸に観客が大盛り上がりしたという意味になります。",
        ];

        for i in inp {
            let sentenec_parser = SentenceAnalyer::new(&analyzer, parser.parse(i));
            let analyzed = sentenec_parser.analyze();
            let mut out = String::new();
            for a in analyzed {
                out.push_str(&a.get_inflected());
            }
            assert_eq!(*i, out);
        }
    }
}
*/
