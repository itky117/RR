main <- function() {
  a <- matrix(seq_len(12L), 3L, 4L)
  b <- matrix(seq_len(12L) + 1L, 3L, 4L)
  c <- a + b
  d0 <- sum(abs(c - (a + b)))

  d1 <- sum(rowSums(a)) - sum(a)
  d2 <- sum(colSums(a)) - sum(a)
  d3 <- a[2L, 3L] - 8L

  print(d0)
  print(d1)
  print(d2)
  print(d3)

  d0 + abs(d1) + abs(d2) + abs(d3)
}

print(main())
