main <- function() {
  x <- c(TRUE, NA, FALSE)
  y <- x & TRUE
  z <- x | FALSE
  print(y)
  print(z)
  0L
}

print(main())
