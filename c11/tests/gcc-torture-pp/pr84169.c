# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr84169.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr84169.c" 2



typedef unsigned __int128 T;




T b;

static __attribute__ ((noipa)) T
foo (T c, T d, T e, T f, T g, T h)
{
  __builtin_mul_overflow ((unsigned char) h, -16, &h);
  return b + h;
}

int
main ()
{
  T x = foo (0, 0, 0, 0, 0, 4);
  if (x != -64)
    __builtin_abort ();
  return 0;
}
