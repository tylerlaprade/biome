use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxFragment;
use rome_rowan::AstNode;

impl ToFormatElement for JsxFragment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}