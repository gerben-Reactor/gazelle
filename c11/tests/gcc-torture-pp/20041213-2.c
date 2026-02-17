# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041213-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041213-2.c" 2





extern void abort (void) __attribute__((noreturn));
extern void exit (int) __attribute__((noreturn));

void __attribute__((noinline))
foo (int i)
{
  int next_n = 1;
  int j = 0;

  for (; i != 0; i--)
    {
      int n;

      for (n = next_n; j < n; j++)
 next_n++;

      if (j != n)
 abort ();
    }
}

int
main (void)
{
  foo (2);
  exit (0);
}
