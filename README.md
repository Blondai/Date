# Date

This is a small implementation of all things date related.

The use of Rusts type system will enforce ergonomic error handling and propagation.

There are own structs and enums respectively for `day`, `month` and `year` to enforce the existence of only valid dates.
Additionally, a struct for the `Age` of a person is added to ensure that an age calculated with this lib is in logical bounds.

# Rata Temporis

Based on §2 of the german ["Gesetz zur Verbesserung der betrieblichen Altersversorgung"](https://www.gesetze-im-internet.de/betravg/__2.html)
`Rata Temporis` ("m/n-tel") is the quotient of the actual service time (`m`) to the possible service time until the legal or contractual pension age (`n`).
