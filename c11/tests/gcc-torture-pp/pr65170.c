# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65170.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65170.c" 2



typedef unsigned __int128 V;
typedef unsigned long long int H;





__attribute__((noinline, noclone)) void
foo (V b, V c)
{
  V a;
  b &= (H) -1;
  c &= (H) -1;
  a = b * c;
  if (a != 1)
    __builtin_abort ();
}

int
main ()
{
  foo (1, 1);
  return 0;
}
