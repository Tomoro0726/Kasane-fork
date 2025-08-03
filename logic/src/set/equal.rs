//! Equality (`==`) logic for `SpaceTimeIdSet`.

use crate::set::SpaceTimeIdSet;

impl PartialEq for SpaceTimeIdSet {
    /// Determines whether two `SpaceTimeIdSet` instances represent the same
    /// physical space-time region.
    ///
    /// The comparison does **not** depend on the internal structure or number of
    /// `SpaceTimeId` entries within each set. Instead, it checks whether the actual
    /// physical areas covered by the two sets are equivalent.
    ///
    /// This is done by computing the difference between `self` and `other` using
    /// set complement and intersection operations. If the remaining area in `self`
    /// (i.e., the part not covered by `other`) is empty, then both sets are considered equal.
    ///
    fn eq(&self, other: &Self) -> bool {
        let stid = (!other.clone()) & self.clone();
        stid.is_empty()
    }
}
