add1 <- function(x) {
  x + 1L
}

apply_twice <- function(f, x) {
  f(f(x))
}

main <- function() {
  rec <- list(a = 3L, b = 4L)
  rec$a <- rec$a + 2L

  seed <- 10L
  add_seed <- function(v) v + seed

  r1 <- add1(4L)
  r2 <- (r1 + 1L)
  r3 <- apply_twice(function(v) v + 1L, 3L)
  r4 <- add_seed(5L)
  r5 <- (function(a) a * 2L)(6L)

  print(rec$a)
  print(rec$b)
  print(r1)
  print(r2)
  print(r3)
  print(r4)
  print(r5)

  rec$a + rec$b + r1 + r2 + r3 + r4 + r5
}

print(main())
