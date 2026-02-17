# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85169.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85169.c" 2


typedef char V __attribute__((vector_size (64)));

static void __attribute__ ((noipa))
foo (V *p)
{
  V v = *p;
  v[63] = 1;
  *p = v;
}

int
main ()
{
  V v = (V) { };
  foo (&v);
  for (unsigned i = 0; i < 64; i++)
    if (v[i] != (i == 63))
      __builtin_abort ();
  return 0;
}
