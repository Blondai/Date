# Date

This is a small implementation of all things date related.

The use of Rusts type system will enforce ergonomic error handling and propagation.

There are own structs and enums respectively for `day`, `month` and `year` to enforce the existence of only valid dates.
Additionally, a struct for the `Age` of a person is added to ensure that an age calculated with this lib is in logical bounds.

# Rata Temporis

Based on §2 of the german ["Gesetz zur Verbesserung der betrieblichen Altersversorgung"](https://www.gesetze-im-internet.de/betravg/__2.html)
`Rata Temporis` ("m/n-tel") is the quotient of the actual service time (`m`) to the possible service time until the legal or contractual pension age (`n`).

As of the implementation of the `day`-, `month`- and `year`-difference inside the `Date` part of this library
this currently only counts **full** differences.
Therefore, the `month_difference` from `01.01.2024` to `31.01.2025` will be 12 months although its (almost) 13.
There should probably be a `Rounding` struct with different options like `Ceil`, `Floor` and `Mathematical`.
