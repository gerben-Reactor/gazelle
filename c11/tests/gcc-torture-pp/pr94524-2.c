# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94524-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94524-2.c" 2


typedef signed char __attribute__ ((__vector_size__ (16))) V;

static __attribute__ ((__noinline__, __noclone__)) V
foo (V c)
{
  c %= (signed char) -128;
  return (V) c;
}

int
main ()
{
  V x = foo ((V) { -128 });
  if (x[0] != 0)
    __builtin_abort ();
  x = foo ((V) { -127 });
  if (x[0] != -127)
    __builtin_abort ();
  x = foo ((V) { 127 });
  if (x[0] != 127)
    __builtin_abort ();
  return 0;
}
