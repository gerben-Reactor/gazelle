# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931018-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931018-1.c" 2


unsigned int a[0x1000];
extern const unsigned long v;

main ()
{
  f (v);
  f (v);
  exit (0);
}

f (a)
     unsigned long a;
{
  if (a != 0xdeadbeefL)
    abort();
}

const unsigned long v = 0xdeadbeefL;
