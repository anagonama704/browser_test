//! integrity-core: 共通モデル定義
//!
//! このクレートは、ハード制約チェッカーの共通データ型を提供します。

mod constraint;
mod error;
mod report;

pub use constraint::{Constraint, ConstraintId, Severity};
pub use error::IntegrityError;
pub use report::{Evidence, Report, Violation};
