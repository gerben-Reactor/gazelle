# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70222-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70222-2.c" 2



__attribute__((noinline, noclone)) unsigned int
foo (int x)
{
  unsigned long long y = -1ULL >> x;
  return (unsigned int) y >> 31;
}


int
main ()
{

  if (foo (15) != 1 || foo (32) != 1 || foo (33) != 0)
    __builtin_abort ();

  return 0;
}
