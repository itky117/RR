main <- function() {
  x <- c(1L, NA, 3L)
  l <- c(TRUE, NA, FALSE)
  i <- NA

  print(x + 1L)
  print(x > 1L)
  print(l & TRUE)
  print(l | FALSE)
  print(x[i])

  0L
}

print(main())
