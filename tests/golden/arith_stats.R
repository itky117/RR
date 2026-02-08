main <- function() {
  x <- seq_len(10L)
  y <- x + 1L
  z <- (x * 2L) - 3L

  print(sum(x))
  print(mean(x))
  print(min(z))
  print(max(z))
  print(round(log10(1000L)))
  print(sqrt(16L))
  print(atan2(1L, 1L) > 0L)
  print(sum(abs((x + x) - (x * 2L))))

  sum(y)
}

print(main())
