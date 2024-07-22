# Crust of Rust: Sorting Algorithms

See Jon Gjengset's [youtube video](https://youtu.be/h4RkCyJyXmM?si=HVQ5CdsIU6NtRfcX)

To benchmark and plot (you'll need [R] and [ggplot2]):

```console 
$ cargo r --release > values.dat
$ R
t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# to plot # comparisons
the_plot <- ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_smooth()
the_plot
```

## Notes
- [50:53](https://youtu.be/h4RkCyJyXmM?si=5NxAFVZfYYDkQhpD&t=3053) Changing the `XX::sort` to `XX.srt` was interesting.  Essentially we were changing from an associated method of the type to a method of the type

## References
- [wikipedia - sorting algorithms](https://en.wikipedia.org/wiki/Sorting_algorithm)
- [R](https://www.r-project.org/)
- [ggplot2](https://ggplot2.tidyverse.org/)
