# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94412.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94412.c" 2


typedef unsigned V __attribute__ ((__vector_size__ (sizeof (unsigned) * 2)));

void
foo (V *v, V *w)
{
  *w = -*v / 11;
}

void
bar (V *v, V *w)
{
  *w = -18 / -*v;
}

int
main ()
{
  V a = (V) { 1, 0 };
  V b = (V) { 3, 2147483647 };
  V c, d;
  foo (&a, &c);
  bar (&b, &d);
  if (c[0] != -1U / 11 || c[1] != 0 || d[0] != 0 || d[1] != -18U / -2147483647)
    __builtin_abort ();
  return 0;
}
