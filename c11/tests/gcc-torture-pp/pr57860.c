# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57860.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57860.c" 2


extern void abort (void);
int a, *b = &a, c, d, e, *f = &e, g, *h = &d, k[1] = { 1 };

int
foo (int p)
{
  for (;; g++)
    {
      for (; c; c--);
      *f = *h = p > ((0x1FFFFFFFFLL ^ a) & *b);
      if (k[g])
 return 0;
    }
}

int
main ()
{
  foo (1);
  if (d != 1)
    abort ();
  return 0;
}
