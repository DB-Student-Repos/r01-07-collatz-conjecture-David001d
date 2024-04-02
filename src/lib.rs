 pub fn collatz(mut n: u64) -> Option<u64> {
  let mut count = 0;
  while n != 1 {
    count += 1;
    if n % 2 == 0 {
      n /= 2;
    } else {
      // Check for potential overflow before multiplication
      if n > (u64::MAX / 3) {
        return None; // Overflow would occur
      }
      n = 3 * n + 1;
    }
  }
  Some(count)
}
