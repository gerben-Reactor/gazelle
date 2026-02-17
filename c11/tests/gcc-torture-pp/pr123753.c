# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr123753.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr123753.c" 2


typedef int V __attribute__((__vector_size__ (8)));
typedef short W __attribute__((__vector_size__ (8)));

union { unsigned short u[4]; W w; } u;
V v;

V
foo ()
{
  u.w--;
  V r = v + u.u[0];
  return r;
}

int
main ()
{
  if (sizeof (int) != 4 || sizeof (short) != 2)
    return 0;
  V x = foo ();
  if (x[0] != (unsigned short) -1 || x[1] != (unsigned short) -1)
    __builtin_abort ();
}
