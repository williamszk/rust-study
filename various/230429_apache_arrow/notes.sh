# Those notes are based on:
# https://levelup.gitconnected.com/introduction-to-apache-arrow-with-rust-394f391ea455

cargo new experiment-with-arrow

# Other libraries are provided as companions to Apache Arrow. 
# They provide common functionality that you wouldn’t want to implement yourself. 
# Two Rust-specific libraries I found helpful were DataFusion and Arrow Flight. 
# DataFusion is a query engine built on Apache Arrow that provides data frame and 
# SQL query APIs. Arrow Flight is a serialization library that is used for 
# transporting data across a network. I’ll go into more detail about these 
# libraries in later articles.

# Take a look at DataFusion and Arrow Flight

