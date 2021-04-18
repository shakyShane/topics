use crate::doc_src::MdSrc;
use crate::items::{Item, LineMarker};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

///
/// A container for all error types related to the static analysis
/// of the graph
///
#[derive(Debug, thiserror::Error)]
pub enum DbError<'a> {
    #[error("{}", .0)]
    Cycle(ErrorRef<'a, CycleError>),
}

pub trait IntoDbError<'a> {
    fn into_db_error(self, src: &'a MdSrc<'a>, item: &'a Item) -> DbError<'a>;
}

#[derive(Debug)]
pub struct ErrorRef<'a, T>
where
    T: Debug,
{
    pub inner: T,
    pub item: &'a Item,
    pub src: &'a MdSrc<'a>,
}

impl<T: Debug> Deref for ErrorRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait ErrCode {
    const CODE: &'static str;
}

impl<Err> Display for ErrorRef<'_, Err>
where
    Err: Debug + ErrCode + Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let input = self.src.md_doc_src.input_file.as_ref();
        let _ = write!(
            f,
            "error[{}] {}",
            Err::CODE,
            if input.is_some() {
                input.unwrap().display().to_string()
            } else {
                String::from("")
            }
        );
        let _ = writeln!(f);
        write!(f, "\t{}", self.inner)
    }
}

#[derive(Debug)]
pub struct CycleError {
    pub from: String,
    pub to: LineMarker<String>,
}

impl CycleError {
    pub fn new(from: impl Into<String>, to: LineMarker<String>) -> Self {
        Self {
            from: from.into(),
            to,
        }
    }
}

impl ErrCode for CycleError {
    const CODE: &'static str = "001";
}

impl Display for CycleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let _ = write!(
            f,
            "Infinite loop detected `{}` -> `{}` -> `{}` -> ∞",
            self.from, self.to.item, self.from
        );

        let _ = writeln!(f);

        write!(
            f,
            "\t check line {}",
            self.to.line_start.expect("cannot be absent")
        )
    }
}

impl<'a> IntoDbError<'a> for CycleError {
    fn into_db_error(self, src: &'a MdSrc<'a>, item: &'a Item) -> DbError<'a> {
        DbError::Cycle(ErrorRef {
            inner: self,
            item,
            src,
        })
    }
}
