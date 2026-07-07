use crate::app::cmd::CmdContext;

pub struct EvalPair<T> {
    pub result: T,
    pub next_idx: usize,
}

impl CmdContext<'_> {
    pub fn eval_logic(&self, idx: usize) -> EvalPair<bool> {
        EvalPair {
            result: self.token_expansion_at(idx) != "",
            next_idx: idx + 1,
        }
    }
    
    pub fn eval_expression(&self, idx: usize) -> EvalPair<String> {
        EvalPair {
            result: self.token_expansion_at(idx),
            next_idx: idx + 1,
        }
    }
}