# Date

This is a small implementation of all things date related.

The use of Rusts type system will enforce ergonomic error handling and propagation.

There are own structs and enums respectively for `day`, `month` and `year` to enforce the existence of only valid dates.
Additionally, a struct for the `Age` of a person is added to ensure that an age calculated with this lib is in logical bounds.
