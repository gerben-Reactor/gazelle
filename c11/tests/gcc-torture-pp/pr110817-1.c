# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110817-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110817-1.c" 2
typedef unsigned long __attribute__((__vector_size__ (8))) V;


V c;

int
main (void)
{
  V v = ~((V) { } <=0);
  if (v[0])
    __builtin_abort ();
  return 0;
}
