use crate::parser::CssParser;
use crate::syntax::parse_error::expected_selector;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{eat_or_recover_selector_function_close_token, CssSelectorList};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![MATCHES_KW, NOT_KW, IS_KW, WHERE_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_selector_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need to check if the identifier is valid, because we already did that
    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let list = CssSelectorList::default()
        .with_end_kind(T![')'])
        // we don't need to recover here, because we have a better diagnostic message in a close token
        .disable_recovery()
        .parse_list(p);
    let list_range = list.range(p);

    if list_range.is_empty() && p.at(T![')']) {
        let diagnostic = expected_selector(p, list_range);
        p.error(diagnostic);
    }

    let kind = if eat_or_recover_selector_function_close_token(p, list, expected_selector)
        && !list_range.is_empty()
    {
        CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}
