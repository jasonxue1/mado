use miette::Result;

use crate::config::lint::RuleSet;
use crate::config::Config;
use crate::rule::{
    MD001, MD002, MD003, MD004, MD005, MD006, MD007, MD009, MD010, MD012, MD013, MD014, MD018,
    MD019, MD020, MD021, MD022, MD023, MD024, MD025, MD026, MD027, MD028, MD029, MD030, MD031,
    MD032, MD033, MD034, MD035, MD036, MD037, MD038, MD039, MD040, MD041, MD046, MD047,
};
use crate::violation::Violation;
use crate::Document;
use crate::Rule;

#[derive(Default)]
pub struct Linter {
    rules: Vec<Rule>,
}

impl Linter {
    #[inline]
    #[must_use]
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    #[inline]
    pub fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        // Iterate rules while unrolling Vec<Result<Vec<..>>> to Result<Vec<..>>
        self.rules.iter().try_fold(vec![], |mut unrolled, rule| {
            let result = rule.check(doc);
            unrolled.extend(result?);
            Ok(unrolled)
        })
    }
}

impl From<&Config> for Linter {
    #[inline]
    #[must_use]
    fn from(config: &Config) -> Self {
        let rules: Vec<_> = config
            .lint
            .rules
            .iter()
            .map(|rule| match rule {
                RuleSet::MD001 => Rule::MD001(MD001::new()),
                RuleSet::MD002 => Rule::MD002(MD002::from(&config.lint.md002)),
                RuleSet::MD003 => Rule::MD003(MD003::from(&config.lint.md003)),
                RuleSet::MD004 => Rule::MD004(MD004::from(&config.lint.md004)),
                RuleSet::MD005 => Rule::MD005(MD005::new()),
                RuleSet::MD006 => Rule::MD006(MD006::new()),
                RuleSet::MD007 => Rule::MD007(MD007::from(&config.lint.md007)),
                RuleSet::MD009 => Rule::MD009(MD009::new()),
                RuleSet::MD010 => Rule::MD010(MD010::new()),
                RuleSet::MD012 => Rule::MD012(MD012::new()),
                RuleSet::MD013 => Rule::MD013(MD013::from(&config.lint.md013)),
                RuleSet::MD014 => Rule::MD014(MD014::new()),
                RuleSet::MD018 => Rule::MD018(MD018::new()),
                RuleSet::MD019 => Rule::MD019(MD019::new()),
                RuleSet::MD020 => Rule::MD020(MD020::new()),
                RuleSet::MD021 => Rule::MD021(MD021::new()),
                RuleSet::MD022 => Rule::MD022(MD022::new()),
                RuleSet::MD023 => Rule::MD023(MD023::new()),
                RuleSet::MD024 => Rule::MD024(MD024::new()),
                RuleSet::MD025 => Rule::MD025(MD025::from(&config.lint.md025)),
                RuleSet::MD026 => Rule::MD026(MD026::from(&config.lint.md026)),
                RuleSet::MD027 => Rule::MD027(MD027::new()),
                RuleSet::MD028 => Rule::MD028(MD028::new()),
                RuleSet::MD029 => Rule::MD029(MD029::from(&config.lint.md029)),
                RuleSet::MD030 => Rule::MD030(MD030::from(&config.lint.md030)),
                RuleSet::MD031 => Rule::MD031(MD031::new()),
                RuleSet::MD032 => Rule::MD032(MD032::new()),
                RuleSet::MD033 => Rule::MD033(MD033::from(&config.lint.md033)),
                RuleSet::MD034 => Rule::MD034(MD034::new()),
                RuleSet::MD035 => Rule::MD035(MD035::from(&config.lint.md035)),
                RuleSet::MD036 => Rule::MD036(MD036::from(&config.lint.md036)),
                RuleSet::MD037 => Rule::MD037(MD037::new()),
                RuleSet::MD038 => Rule::MD038(MD038::new()),
                RuleSet::MD039 => Rule::MD039(MD039::new()),
                RuleSet::MD040 => Rule::MD040(MD040::new()),
                RuleSet::MD041 => Rule::MD041(MD041::from(&config.lint.md041)),
                RuleSet::MD046 => Rule::MD046(MD046::from(&config.lint.md046)),
                RuleSet::MD047 => Rule::MD047(MD047::new()),
            })
            .collect();

        Self { rules }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, Arena};
    use pretty_assertions::assert_eq;

    use crate::rule::RuleLike as _;
    use crate::rule::{
        md003::HeadingStyle, md004::ListStyle, md029::OrderedListStyle, md035::HorizontalRuleStyle,
        md046::CodeBlockStyle,
    };

    use super::*;

    #[test]
    fn from_md001() {
        let md001 = Rule::MD001(MD001::new());
        let rules = vec![RuleSet::MD001];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md001]);
    }

    #[test]
    fn from_md002() {
        let level = 3;
        let md002 = Rule::MD002(MD002::new(level));
        let rules = vec![RuleSet::MD002];
        let mut config = Config::default();
        config.lint.md002.level = level;
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md002]);
    }

    #[test]
    fn from_md003() {
        let style = HeadingStyle::SetextWithAtx;
        let md003 = Rule::MD003(MD003::new(style.clone()));
        let rules = vec![RuleSet::MD003];
        let mut config = Config::default();
        config.lint.md003.style = style;
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md003]);
    }

    #[test]
    fn from_md004() {
        let style = ListStyle::Asterisk;
        let md004 = Rule::MD004(MD004::new(style.clone()));
        let rules = vec![RuleSet::MD004];
        let mut config = Config::default();
        config.lint.md004.style = style;
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md004]);
    }

    #[test]
    fn from_md005() {
        let md005 = Rule::MD005(MD005::new());
        let rules = vec![RuleSet::MD005];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md005]);
    }

    #[test]
    fn from_md006() {
        let md006 = Rule::MD006(MD006::new());
        let rules = vec![RuleSet::MD006];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md006]);
    }

    #[test]
    fn from_md007() {
        let indent = 9;
        let md007 = Rule::MD007(MD007::new(indent));
        let rules = vec![RuleSet::MD007];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md007.indent = indent;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md007]);
    }

    #[test]
    fn from_md009() {
        let md009 = Rule::MD009(MD009::new());
        let rules = vec![RuleSet::MD009];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md009]);
    }

    #[test]
    fn from_md010() {
        let md010 = Rule::MD010(MD010::new());
        let rules = vec![RuleSet::MD010];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md010]);
    }

    #[test]
    fn from_md012() {
        let md012 = Rule::MD012(MD012::new());
        let rules = vec![RuleSet::MD012];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md012]);
    }

    #[test]
    fn from_md013() {
        let line_length = 33;
        let code_blocks = false;
        let tables = false;
        let md013 = Rule::MD013(MD013::new(line_length, code_blocks, tables));
        let rules = vec![RuleSet::MD013];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md013.line_length = line_length;
        config.lint.md013.code_blocks = code_blocks;
        config.lint.md013.tables = tables;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md013]);
    }

    #[test]
    fn from_md014() {
        let md014 = Rule::MD014(MD014::new());
        let rules = vec![RuleSet::MD014];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md014]);
    }

    #[test]
    fn from_md018() {
        let md018 = Rule::MD018(MD018::new());
        let rules = vec![RuleSet::MD018];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md018]);
    }

    #[test]
    fn from_md019() {
        let md019 = Rule::MD019(MD019::new());
        let rules = vec![RuleSet::MD019];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md019]);
    }

    #[test]
    fn from_md020() {
        let md020 = Rule::MD020(MD020::new());
        let rules = vec![RuleSet::MD020];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md020]);
    }

    #[test]
    fn from_md021() {
        let md021 = Rule::MD021(MD021::new());
        let rules = vec![RuleSet::MD021];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md021]);
    }

    #[test]
    fn from_md022() {
        let md022 = Rule::MD022(MD022::new());
        let rules = vec![RuleSet::MD022];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md022]);
    }

    #[test]
    fn from_md023() {
        let md023 = Rule::MD023(MD023::new());
        let rules = vec![RuleSet::MD023];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md023]);
    }

    #[test]
    fn from_md024() {
        let md024 = Rule::MD024(MD024::new());
        let rules = vec![RuleSet::MD024];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md024]);
    }

    #[test]
    fn from_md025() {
        let level = 3;
        let md025 = Rule::MD025(MD025::new(level));
        let rules = vec![RuleSet::MD025];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md025.level = level;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md025]);
    }

    #[test]
    fn from_md027() {
        let md027 = Rule::MD027(MD027::new());
        let rules = vec![RuleSet::MD027];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md027]);
    }

    #[test]
    fn from_md028() {
        let md028 = Rule::MD028(MD028::new());
        let rules = vec![RuleSet::MD028];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md028]);
    }

    #[test]
    fn from_md029() {
        let style = OrderedListStyle::Ordered;
        let md029 = Rule::MD029(MD029::new(style.clone()));
        let rules = vec![RuleSet::MD029];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md029.style = style;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md029]);
    }

    #[test]
    fn from_md030() {
        let ul_single = 1;
        let ol_single = 2;
        let ul_multi = 3;
        let ol_multi = 4;
        let md030 = Rule::MD030(MD030::new(ul_single, ol_single, ul_multi, ol_multi));
        let rules = vec![RuleSet::MD030];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md030.ul_single = ul_single;
        config.lint.md030.ol_single = ol_single;
        config.lint.md030.ul_multi = ul_multi;
        config.lint.md030.ol_multi = ol_multi;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md030]);
    }

    #[test]
    fn from_md031() {
        let md031 = Rule::MD031(MD031::new());
        let rules = vec![RuleSet::MD031];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md031]);
    }

    #[test]
    fn from_md032() {
        let md032 = Rule::MD032(MD032::new());
        let rules = vec![RuleSet::MD032];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md032]);
    }

    #[test]
    fn from_md033() {
        let allowed_elements = vec!["br".to_owned()];
        let md033 = Rule::MD033(MD033::new(&allowed_elements));
        let rules = vec![RuleSet::MD033];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md033.allowed_elements = allowed_elements;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md033]);
    }

    #[test]
    fn from_md034() {
        let md034 = Rule::MD034(MD034::new());
        let rules = vec![RuleSet::MD034];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md034]);
    }

    #[test]
    fn from_md035() {
        let style = HorizontalRuleStyle::Custom("~~~".to_owned());
        let md035 = Rule::MD035(MD035::new(style.clone()));
        let rules = vec![RuleSet::MD035];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md035.style = style;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md035]);
    }

    #[test]
    fn from_md036() {
        let punctuation = "!?".to_owned();
        let md036 = Rule::MD036(MD036::new(punctuation.clone()));
        let rules = vec![RuleSet::MD036];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md036.punctuation = punctuation;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md036]);
    }

    #[test]
    fn from_md037() {
        let md037 = Rule::MD037(MD037::new());
        let rules = vec![RuleSet::MD037];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md037]);
    }

    #[test]
    fn from_md038() {
        let md038 = Rule::MD038(MD038::new());
        let rules = vec![RuleSet::MD038];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md038]);
    }

    #[test]
    fn from_md039() {
        let md039 = Rule::MD039(MD039::new());
        let rules = vec![RuleSet::MD039];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md039]);
    }

    #[test]
    fn from_md040() {
        let md040 = Rule::MD040(MD040::new());
        let rules = vec![RuleSet::MD040];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md040]);
    }

    #[test]
    fn from_md041() {
        let level = 3;
        let md041 = Rule::MD041(MD041::new(level));
        let rules = vec![RuleSet::MD041];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md041.level = level;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md041]);
    }

    #[test]
    fn from_md046() {
        let style = CodeBlockStyle::Indented;
        let md046 = Rule::MD046(MD046::new(style.clone()));
        let rules = vec![RuleSet::MD046];
        let mut config = Config::default();
        config.lint.rules = rules;
        config.lint.md046.style = style;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md046]);
    }

    #[test]
    fn from_md047() {
        let md047 = Rule::MD047(MD047::new());
        let rules = vec![RuleSet::MD047];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        assert_eq!(linter.rules, vec![md047]);
    }

    #[test]
    fn check_with_front_matter() {
        let text = "---
comments: false
description: Some text
---

# This is a header."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let md026 = MD026::default();
        let rules = vec![RuleSet::MD026];
        let mut config = Config::default();
        config.lint.rules = rules;
        let linter = Linter::from(&config);
        let actual = linter.check(&doc).unwrap();
        let expected = vec![md026.to_violation(path, Sourcepos::from((6, 1, 6, 19)))];
        assert_eq!(actual, expected);
    }
}
