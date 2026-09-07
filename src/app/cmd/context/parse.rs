use std::any;
use std::str::FromStr;
use crate::error::PanicResult;
use crate::lexer;
use crate::types::ShapeParse;
use super::CmdContext;

impl CmdContext<'_, '_> {
    pub fn parse_token<TShape: ShapeParse>(&self, idx: usize) -> PanicResult<TShape::Output> {
        let expansion = &self.token_expansion_at(idx);
        
        let Some(output) = TShape::parse(expansion) else {
            let expansion_definition = lexer::get_definition_display(expansion);
            let type_name = any::type_name::<TShape::Output>();

            self.raise_at_token(format!(
                "Value {} cannot be converted into {}",
                expansion_definition, type_name
            ), idx)?
        };
        
        Ok(output)
    }
}