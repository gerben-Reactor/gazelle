# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41935.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41935.c" 2


extern void abort (void);

long int
foo (int n, int i, int j)
{
  typedef int T[n];
  struct S { int a; T b[n]; };
  return __builtin_offsetof (struct S, b[i][j]);
}

int
main (void)
{
  typedef int T[5];
  struct S { int a; T b[5]; };
  if (foo (5, 2, 3)
      != __builtin_offsetof (struct S, b) + (5 * 2 + 3) * sizeof (int))
    abort ();
  if (foo (5, 5, 5)
      != __builtin_offsetof (struct S, b) + (5 * 5 + 5) * sizeof (int))
    abort ();
  return 0;
}
