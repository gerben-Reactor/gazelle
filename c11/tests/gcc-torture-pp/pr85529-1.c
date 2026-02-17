# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85529-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85529-1.c" 2


struct S { int a; };

int b, c = 1, d, e, f;
static int g;
volatile struct S s;

signed char
foo (signed char i, int j)
{
  return i < 0 ? i : i << j;
}

int
main ()
{
  signed char k = -83;
  if (!d)
    goto L;
  k = e || f;
L:
  for (; b < 1; b++)
    s.a != (k < foo (k, 2) && (c = k = g));
  if (c != 1)
    __builtin_abort ();
  return 0;
}
