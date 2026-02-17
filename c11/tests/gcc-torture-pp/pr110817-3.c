# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110817-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110817-3.c" 2
typedef unsigned __attribute__((__vector_size__ (1*sizeof(unsigned)))) V;

V v;
unsigned char c;

int
main (void)
{
  V x = (v > 0) > (v != c);
  volatile signed int t = x[0];
  if (t)
    __builtin_abort ();
  return 0;
}
