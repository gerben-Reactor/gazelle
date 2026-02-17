# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr108789.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr108789.c" 2


int
add (unsigned *r, const unsigned *a, const unsigned *b)
{
  return __builtin_add_overflow (*a, *b, r);
}

int
mul (unsigned *r, const unsigned *a, const unsigned *b)
{
  return __builtin_mul_overflow (*a, *b, r);
}

int
main ()
{
  unsigned x;


  x = (2147483647 + 1U) / 2;
  if (add (&x, &x, &x))
    __builtin_abort ();


  x = 1U << (sizeof (int) * 8 / 4);
  if (mul (&x, &x, &x))
    __builtin_abort ();


  x = 2147483647 + 1U;
  if (!add (&x, &x, &x))
    __builtin_abort ();


  x = 1U << (sizeof (int) * 8 / 2);
  if (!mul (&x, &x, &x))
    __builtin_abort ();
}
