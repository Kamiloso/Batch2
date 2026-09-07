use crate::app::cmd::CmdContext;
use crate::error::RuntimeResult;
use crate::lexer;
use crate::tools::operators;
use crate::types::{ShapeParse, ShapeStringify, BoolShape};

pub struct EvalPair<T> {
    pub result: T,
    pub next_idx: usize,
}

impl CmdContext<'_, '_> {
    pub fn eval_join(&self, idx: usize) -> RuntimeResult<EvalPair<String>> {
        let mut builder = String::new();
        let len = self.tokens_len();
        
        let mut i = idx;
        while i < len {
            let next = self.token_expansion_at(i);
            builder.push_str(&next);
            
            if i != len - 1 {
                builder.push(' ');
            }

            i += 1;
        }
        
        Ok(EvalPair {
            result: builder,
            next_idx: i,
        })
    }
    
    pub fn eval_expression(&self, idx: usize) -> RuntimeResult<EvalPair<String>> {
        // TODO: make this work seriously
        
        let mut builder = String::new();
        let len = self.tokens_len();
        
        let mut i = idx;
        while i < len {
            let next = self.token_expansion_at(i);
            let terminates = next == "END";
            
            if !terminates {
                builder.push_str(&next);
            }
            
            i += 1;
            
            if terminates {
                break;
            }
        }
        
        Ok(EvalPair {
            result: builder,
            next_idx: i,
        })
    }

    pub fn eval_logic(&self, idx: usize) -> RuntimeResult<EvalPair<bool>> {
        let expression_pair = self.eval_expression(idx)?;

        let Some(bool_parsed) = BoolShape::parse(&expression_pair.result) else {
            self.raise_raw(format!(
                "Expression {} must be a bool",
                lexer::get_definition_display(&expression_pair.result),
            ))?
        };

        Ok(EvalPair {
            result: bool_parsed,
            next_idx: expression_pair.next_idx,
        })
    }
}