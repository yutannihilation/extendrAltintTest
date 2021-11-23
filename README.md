
<!-- README.md is generated from README.Rmd. Please edit that file -->

# extendrAltintTest

This repository is for illustrating the problem of extendr
([extendr/extendr#310](https://github.com/extendr/extendr/issues/310#issuecomment-950250792))

``` r
library(extendrAltintTest)

x <- get_intrange()

# works
x[3]
#> [1] 2

# won't work yet
x[2:3]
#> NULL
```
