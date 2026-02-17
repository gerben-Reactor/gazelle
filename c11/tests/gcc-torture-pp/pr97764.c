# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97764.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97764.c" 2



struct S { int b : 3; int c : 28; int d : 1; };

int
main ()
{
  struct S e = {};
  e.c = -1;
  if (e.d)
    __builtin_abort ();
  return 0;
}
