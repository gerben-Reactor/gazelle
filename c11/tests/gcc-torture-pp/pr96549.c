# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr96549.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr96549.c" 2


long c = -1L;
long b = 0L;

int
main ()
{
  if (3L > (short) ((c ^= (b = 1L)) * 3L))
    return 0;
  __builtin_abort ();
}
