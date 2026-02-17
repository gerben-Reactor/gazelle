# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71626-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71626-1.c" 2


typedef long int V __attribute__((__vector_size__(sizeof (long int))));

__attribute__((noinline, noclone)) V
foo ()
{
  V v = { (long int) foo };
  return v;
}

int
main ()
{
  V v = foo ();
  if (v[0] != (long int) foo)
    __builtin_abort ();
  return 0;
}
