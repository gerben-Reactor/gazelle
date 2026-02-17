# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94524-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94524-1.c" 2


typedef signed char __attribute__ ((__vector_size__ (16))) V;

static __attribute__ ((__noinline__, __noclone__)) V
foo (V c)
{
  c %= (signed char) -19;
  return (V) c;
}

int
main ()
{
  V x = foo ((V) { 31 });
  if (x[0] != 12)
    __builtin_abort ();
  return 0;
}
