map_err <- function(n) {
  x <- seq_len(n)
  y <- seq_len(n)
  for (i in seq.int(1L, length(x))) {
    y[i] <- (x[i] * 2L) + 1L
  }
  target <- (x * 2L) + 1L
  sum(abs(y - target))
}

cond_err <- function(n, k) {
  x <- seq_len(n)
  y <- seq_len(n)
  for (i in seq.int(1L, length(x))) {
    if (x[i] > k) {
      y[i] <- x[i]
    } else {
      y[i] <- 0L
    }
  }
  target <- (x > k) * x
  sum(abs(y - target))
}

main <- function() {
  e1 <- map_err(20L)
  e2 <- cond_err(20L, 8L)
  print(e1)
  print(e2)
  e1 + e2
}

print(main())
