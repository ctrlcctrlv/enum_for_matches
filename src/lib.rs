/// enum_for_matches runs a match arm for each enum variant passed into it regardless of type. So,
/// you can make a string out of an enum which wraps numeric types individually, such as
/// `serde_value::Value` for example. See README.md on GitHub for more information.
///
/// (c) 2020 Fredrick R. Brennan. Apache 2: See LICENSE.md.

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

trait SplitPunct {
    fn split_punct(&self, p: char) -> Vec<Vec<TokenTree>>;
}

impl SplitPunct for TokenStream {
    fn split_punct(&self, p: char) -> Vec<Vec<TokenTree>> {
        let tokenvec: Vec<TokenTree> = self.clone().into_iter().collect();

        tokenvec
            .split(|t| match t {
                TokenTree::Punct(punct) => punct.as_char() == p,
                _ => false,
            })
            .map(|i| i.into_iter().map(|ii| ii.clone()).collect())
            .collect()
    }
}

trait FromVecTokenTree {
    fn make_token_stream(&self) -> TokenStream;
    fn make_token_tree(&self) -> TokenTree;
}

impl FromVecTokenTree for Vec<TokenTree> {
    fn make_token_stream(&self) -> TokenStream {
        let mut t = TokenStream::new();
        t.extend(self.clone());
        t
    }
    fn make_token_tree(&self) -> TokenTree {
        TokenTree::Group(Group::new(Delimiter::Brace, self.make_token_stream()))
    }
}

fn begin(match_on: &Vec<TokenTree>) -> TokenStream {
    let mut t = TokenStream::new();
    t.extend(vec![
        "match ".parse::<TokenStream>().unwrap(),
        match_on.clone().make_token_stream(),
    ]);
    t
}

thread_local! {
    static ARROW: Vec<TokenTree> = vec![TokenTree::Punct(Punct::new('=', Spacing::Joint)), TokenTree::Punct(Punct::new('>', Spacing::Joint))];
}

#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    let comma_split = input.split_punct(',');
    let mut ret = begin(&comma_split[0]);
    let split_input = match &comma_split[1][0] {
        TokenTree::Group(g) => g.stream().split_punct('|'),
        _ => panic!()
    };

    let run = vec![(&comma_split[2].clone()[0]).clone()];

    let mut inner = Vec::new();
    for match_ in split_input.iter() {
        inner.extend(match_.clone());
        inner.extend(vec![
            TokenTree::Punct(Punct::new('=', Spacing::Joint)), TokenTree::Punct(Punct::new('>', Spacing::Joint))]);
        inner.extend(run.clone());
    }
    if comma_split.len() > 3 {
        inner.extend(vec![TokenTree::Ident(Ident::new("_", Span::call_site()))]);
        #[allow(non_snake_case)]
        inner.extend(ARROW.with(|A|A.clone()));
        let run = vec![(&comma_split[3].clone()[0]).clone()];
        inner.extend(run.clone());
    } else {
        inner.extend("_ => {}".parse::<TokenStream>().unwrap());
    }

    ret.extend(vec![inner.make_token_tree()]);
    eprintln!("{}", &ret.to_string());
    ret
}
