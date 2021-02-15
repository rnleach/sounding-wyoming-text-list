# sounding-wyoming-text-list

Library to parse and iterate over weather soundings retrieved from [University of Wyoming][1].

Retrieve a sounding from the [University of Wyoming][1] page in the "Text: List" format, and
then pass the text to the `parse_text()` function and it will return an iterator. The iterator
returns `(sounding_analysis::Sounding, std::collections::HashMap<&'static str, f64>)` items.

The hash map contains indexes and values provided in the text that are not a part of the
`sounding_analysis::Sounding` type. The keys in the hashmap should describe the values.

You may build a program that automatically downloads the soundings from the website or one that
loads the text from disk, either way just pass the text into the `parse_text()` function and
it will parse it for you.

The crate was originally built to support [sonde][2], a sounding data viewer and analysis tool.

[1]: http://weather.uwyo.edu/upperair/sounding.html
[2]:https://github.com/rnleach/sonde
