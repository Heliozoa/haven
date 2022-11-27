//! Contains functions for doing database operations on domain types, such as posts or tags.

pub mod blogs;
pub mod comics;
pub mod lists;
pub mod posts;
pub mod tags;
pub mod users;

/// Helper macro for making queries.
///
/// eq!(table, column_1, column_2)
/// =
/// (table::column_1.eq(column_1), table::column_2.eq(column_2))
///
/// eq!(table_1::column_1, table_2::column_2)
/// =
/// (table_1::column_1.eq(column_1), table_2::column_2.eq(column_2))
#[macro_export]
macro_rules! eq {
    ($t:ident, $($c: ident),* $(,)?) => {
        ( $($t::$c.eq($c)),* )
    };
    ($($t:ident :: $c: ident),* $(,)?) => {
        ( $($t::$c.eq($c)),* )
    };
}
pub use crate::eq;

/// Helper macro for implementing Queryable and Selectable and ensures the implementations match.
///
/// ```
/// query! {
///     #[derive(Debug, Serialize, Elm, ElmDecode)]
///     pub struct PostSmall {
///         pub id: i32 = posts::id,
///         pub thumbnail: String = posts::thumbnail_filename,
///         pub title: String = posts::title,
///         pub user_id: i32 = users::id,
///         pub username: String = users::display_name,
///     }
/// }
/// ```
#[macro_export]
macro_rules! query {
    (
        $(#[ $attr:meta ])*
        $v:vis $kw:ident $name:ident {
            $(
                $fv:vis $field:ident: $t:ty = $table:ident :: $column:ident
            ),* $(,)?
        }
    ) => {
        $(#[ $attr ])*
        #[derive(::diesel::Queryable)]
        $v $kw $name {
            $($fv $field: $t),*
        }

        impl<DB: ::diesel::backend::Backend> ::diesel::Selectable<DB> for $name {
            type SelectExpression = ($( crate::schema::$table::$column ),*);

            fn construct_selection() -> Self::SelectExpression {
                ($( crate::schema::$table::$column ),*)
            }
        }
    };
}
pub use crate::query;
