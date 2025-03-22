use crate::stringtokeniser::StrToken;
pub type MergedLine = Vec<StrToken>;

pub enum ResolvedToken {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
    Var(f64),
}


