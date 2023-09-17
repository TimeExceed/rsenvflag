#![doc = include_str!("../../README.md")]

use proc_macro::*;
use std::collections::VecDeque;

#[proc_macro_attribute]
pub fn envflag(attr: TokenStream, item: TokenStream) -> TokenStream {
    let flag = FlagContext::new(attr.into_iter().collect(), item.into_iter().collect());
    flag.into()
}

#[derive(Debug, Clone)]
struct FlagContext {
    preamble: Vec<TokenTree>,
    ident: Ident,
    base_type: VecDeque<TokenTree>,
    has_option: bool,
    default: Vec<TokenTree>,
    crate_: Option<Vec<TokenTree>>,
    env_name: Option<Literal>,
    parser: Option<Vec<TokenTree>>,
}

impl FlagContext {
    fn new(mut attr: VecDeque<TokenTree>, mut items: VecDeque<TokenTree>) -> Self {
        let mut default = vec![];
        let mut crate_ = None;
        let mut env_name = None;
        let mut parser = None;
        while let Some(tt) = attr.pop_front() {
            match tt {
                TokenTree::Ident(id) if id.to_string() == "default" => {
                    match attr.pop_front() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
                        _ => {
                            panic!("\"default\" must be assigned a value.");
                        }
                    }
                    collect_until(&mut default, &mut attr, |tt| match tt {
                        TokenTree::Punct(p) if p.as_char() == ',' => true,
                        _ => false,
                    });
                }
                TokenTree::Ident(id) if id.to_string() == "crate" => {
                    match attr.pop_front() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
                        _ => {
                            panic!("\"crate\" must be assigned a name.");
                        }
                    }
                    let mut c = vec![];
                    collect_until(&mut c, &mut attr, |tt| match tt {
                        TokenTree::Punct(p) if p.as_char() == ',' => true,
                        _ => false,
                    });
                    crate_ = Some(c);
                }
                TokenTree::Ident(id) if id.to_string() == "env_name" => {
                    match attr.pop_front() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
                        _ => {
                            panic!("\"env_name\" must be assigned a string literal.");
                        }
                    }
                    let mut c = vec![];
                    collect_until(&mut c, &mut attr, |tt| match tt {
                        TokenTree::Punct(p) if p.as_char() == ',' => true,
                        _ => false,
                    });
                    assert_eq!(
                        c.len(),
                        1,
                        "\"env_name\" must be assigned a string literal.",
                    );
                    match c.pop() {
                        Some(TokenTree::Literal(l)) => {
                            env_name = Some(l);
                        }
                        _ => {
                            panic!("\"env_name\" must be assigned a string literal.");
                        }
                    }
                }
                TokenTree::Ident(id) if id.to_string() == "parser" => {
                    match attr.pop_front() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
                        _ => {
                            panic!("\"parser\" must be assigned a function.");
                        }
                    }
                    let mut c = vec![];
                    collect_until(&mut c, &mut attr, |tt| match tt {
                        TokenTree::Punct(p) if p.as_char() == ',' => true,
                        _ => false,
                    });
                    parser = Some(c);
                }
                _ => {
                    panic!("Unknown attr: {}", tt.to_string());
                }
            }
        }
        let mut preamble = vec![];
        collect_until(&mut preamble, &mut items, |tt| match tt {
            TokenTree::Punct(p) if p.as_char() == ':' => true,
            _ => false,
        })
        .unwrap_or_else(|| {
            panic!("Type is required for an env flag.");
        });
        let Some(TokenTree::Ident(ident)) = preamble.pop() else {
            panic!("A flag needs a name.");
        };
        match items.pop_back() {
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => (),
            _ => {
                panic!("A flag definition must be ended by ';'.");
            }
        }
        let mut base_type: VecDeque<TokenTree> = items.into_iter().collect();
        let has_option = filter_out_option(&mut base_type);
        Self {
            preamble,
            ident,
            base_type,
            has_option,
            default,
            crate_,
            env_name,
            parser,
        }
    }
}

fn collect_until<Pred>(
    out: &mut Vec<TokenTree>,
    input: &mut VecDeque<TokenTree>,
    final_pred: Pred,
) -> Option<TokenTree>
where
    Pred: Fn(&TokenTree) -> bool,
{
    while let Some(tt) = input.pop_front() {
        if final_pred(&tt) {
            return Some(tt);
        }
        out.push(tt);
    }
    None
}

fn filter_out_option(base_type: &mut VecDeque<TokenTree>) -> bool {
    let has_option = base_type
        .iter()
        .take_while(|tt| match tt {
            TokenTree::Punct(p) if p.as_char() == '<' => false,
            _ => true,
        })
        .any(|tt| match tt {
            TokenTree::Ident(id) if id.to_string() == "Option" => true,
            _ => false,
        });
    if has_option {
        match base_type.pop_back() {
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => (),
            _ => {
                panic!("Option has no closing delimiter.");
            }
        }
        let mut not_used = vec![];
        collect_until(&mut not_used, base_type, |tt| match tt {
            TokenTree::Punct(p) if p.as_char() == '<' => true,
            _ => false,
        })
        .unwrap_or_else(|| {
            panic!("Option has no opening delimiter.");
        });
    }
    has_option
}

impl Into<TokenStream> for FlagContext {
    fn into(self) -> TokenStream {
        let mut res = TokenStream::new();
        res.extend(self.preamble.iter().cloned());
        res.extend([
            TokenTree::Ident(self.ident.clone()),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        self.wrapped_type(&mut res);
        res.extend([TokenTree::Punct(Punct::new('=', Spacing::Alone))]);
        self.init_value(&mut res);
        res.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
        res
    }
}

impl FlagContext {
    fn wrapped_type(&self, out: &mut TokenStream) {
        if self.has_option {
            self.wo_default_base(out);
            self.wo_default_type_params(out);
        } else {
            self.w_default_base(out);
            self.w_default_type_params(out);
        }
    }

    fn init_value(&self, out: &mut TokenStream) {
        if self.has_option {
            self.init_wo_default(out);
        } else {
            self.w_default_base(out);
            out.extend([
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ]);
            self.w_default_type_params(out);
            out.extend([self.init_w_default()]);
        }
    }

    fn init_wo_default(&self, out: &mut TokenStream) {
        self.wo_default_base(out);
        out.extend([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        self.wo_default_type_params(out);
        out.extend([{
            let mut res = TokenStream::new();
            res.extend([
                TokenTree::Ident(Ident::new("key", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ]);
            self.env_name(&mut res);
            res.extend([
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                TokenTree::Ident(Ident::new("parser", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ]);
            self.parser(&mut res);
            res.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            let g = Group::new(Delimiter::Brace, res);
            TokenTree::Group(g)
        }]);
    }

    fn init_w_default(&self) -> TokenTree {
        let mut res = TokenStream::new();
        res.extend([
            TokenTree::Ident(Ident::new("env", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        self.init_wo_default(&mut res);
        res.extend([
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("default", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        self.default_value(&mut res);
        res.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        let g = Group::new(Delimiter::Brace, res);
        TokenTree::Group(g)
    }

    fn wo_default_base(&self, out: &mut TokenStream) {
        self.crate_name(out);
        out.extend([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("EnvFlag", Span::mixed_site())),
        ]);
    }

    fn w_default_base(&self, out: &mut TokenStream) {
        self.crate_name(out);
        out.extend([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("EnvFlagWithDefault", Span::mixed_site())),
        ]);
    }

    fn wo_default_type_params(&self, out: &mut TokenStream) {
        out.extend([TokenTree::Punct(Punct::new('<', Spacing::Alone))]);
        out.extend(self.base_type.iter().cloned());
        out.extend([TokenTree::Punct(Punct::new('>', Spacing::Alone))]);
    }

    fn w_default_type_params(&self, out: &mut TokenStream) {
        out.extend([TokenTree::Punct(Punct::new('<', Spacing::Alone))]);
        out.extend(self.base_type.iter().cloned());
        self.default_type_param(out);
        out.extend([TokenTree::Punct(Punct::new('>', Spacing::Alone))]);
    }

    fn default_type_param(&self, out: &mut TokenStream) {
        out.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        match self.base_type.back() {
            Some(TokenTree::Ident(id)) if id.to_string() == "String" => {
                out.extend([
                    TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                    TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("static", Span::mixed_site())),
                    TokenTree::Ident(Ident::new("str", Span::mixed_site())),
                ]);
            }
            Some(tt @ TokenTree::Ident(id))
                if ["i64", "f64", "bool"].contains(&id.to_string().as_str()) =>
            {
                out.extend([tt.clone()]);
            }
            _ => {
                out.extend([
                    TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                    TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("static", Span::mixed_site())),
                ]);
                out.extend(self.base_type.iter().cloned());
            }
        }
    }

    fn crate_name(&self, out: &mut TokenStream) {
        if let Some(ref crate_) = self.crate_ {
            out.extend(crate_.iter().cloned())
        } else {
            out.extend([TokenTree::Ident(Ident::new(
                "rs_envflag",
                Span::mixed_site(),
            ))]);
        }
    }

    fn env_name(&self, out: &mut TokenStream) {
        out.extend([if let Some(ref env_name) = self.env_name {
            TokenTree::Literal(env_name.clone())
        } else {
            TokenTree::Literal(Literal::string(&self.ident.to_string()))
        }])
    }

    fn parser(&self, out: &mut TokenStream) {
        if let Some(ref parser) = self.parser {
            out.extend(parser.iter().cloned());
        } else {
            self.crate_name(out);
            out.extend([
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ]);
            let builtin_parser = match self.base_type.back() {
                Some(TokenTree::Ident(id)) if id.to_string() == "String" => "str_parser",
                Some(TokenTree::Ident(id)) if id.to_string() == "i64" => "i64_parser",
                Some(TokenTree::Ident(id)) if id.to_string() == "f64" => "f64_parser",
                Some(TokenTree::Ident(id)) if id.to_string() == "bool" => "bool_parser",
                _ => {
                    panic!("Unknown base type to parse.");
                }
            };
            out.extend([TokenTree::Ident(Ident::new(
                builtin_parser,
                Span::mixed_site(),
            ))]);
        }
    }

    fn default_value(&self, out: &mut TokenStream) {
        match self.base_type.back() {
            Some(TokenTree::Ident(id)) if id.to_string() == "String" => {
                out.extend([TokenTree::Punct(Punct::new('&', Spacing::Alone))]);
                out.extend(self.default.iter().cloned());
            }
            _ => {
                out.extend(self.default.iter().cloned());
            }
        }
    }
}
