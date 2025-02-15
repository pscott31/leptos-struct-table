use crate::ColumnSort;
use async_trait::async_trait;
use core::fmt::Debug;
use std::collections::VecDeque;
use std::ops::Range;

/// The trait that provides data for the generated table component.
/// Anything that is passed to the `items` prop must implement this trait.
///
/// This is automatically implemented for `Vec<Row>`.
/// This way a simple list of items can be passed to the table.
///
/// Please note that because of the use of [`async-trait`](https://docs.rs/async-trait/latest/async_trait/) this documentation is a bit cluttered.
#[async_trait(?Send)]
pub trait TableDataProvider<Row>
where
    Row: Debug + PartialEq,
{
    /// This is generated by deriving TableComponent. It's an enum with all the column names.
    type ColumnName: Copy;

    /// Get all data rows for the table specified by the range. This method is called when the table is rendered.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case. Use [get_vec_range_clamped] to get a
    /// range that is clamped to the length of the vector.
    async fn get_rows(&self, range: Range<usize>) -> Vec<Row>;

    /// Set the sorting of the table. The sorting is a list of column names and the sort order sorted by priority.
    /// The first entry in the list is the most important one.
    /// The default implementation does nothing.
    /// For example: `[(Column::Name, ColumnSort::Ascending), (Column::Age, ColumnSort::Descending)]`
    /// will sort by name first and then by age.
    /// Please note that after calling this method, data will be reloaded through [`get_rows`](TableDataProvider::get_rows).
    #[allow(unused_variables)]
    fn set_sorting(&mut self, sorting: &VecDeque<(Self::ColumnName, ColumnSort)>) {
        // by default do nothing
    }
}

/// Return `vec[range.start..range.end]` where `range` is clamped to the length of `vec`.
pub fn get_vec_range_clamped<T: Clone>(vec: &Vec<T>, range: Range<usize>) -> Vec<T> {
    if vec.is_empty() {
        return vec![];
    }

    let start = range.start.min(vec.len() - 1);
    let end = range.end.min(vec.len());

    vec[start..end].to_vec()
}
