/// Provides means to map enum  values to positions in arrays backing an EnumMap/EnumTable.
/// Not intended to be implemented by hand. Deriving it with `#[derive(Enumerated)]`
/// attribute macro is preferred.
///
/// ```
/// use enum_collections::Enumerated;
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
/// ```
pub trait Enumerated {
    /// Maps an enum to a unique position in an array.
    fn position(self) -> usize;
    /// Total number of values in an Enum.
    fn len() -> usize;
}
