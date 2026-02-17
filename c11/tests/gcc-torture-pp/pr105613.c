# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105613.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105613.c" 2



typedef unsigned __int128 __attribute__((__vector_size__ (16))) V;

void
foo (V v, V *r)
{
  *r = v != 0;
}

int
main ()
{
  V r;
  foo ((V) {5}, &r);
  if (r[0] != ~(unsigned __int128) 0)
    __builtin_abort ();
  foo ((V) {0x500000005ULL}, &r);
  if (r[0] != ~(unsigned __int128) 0)
    __builtin_abort ();
  foo ((V) {0}, &r);
  if (r[0] != 0)
    __builtin_abort ();
  return 0;
}
