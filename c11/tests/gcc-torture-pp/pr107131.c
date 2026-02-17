# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr107131.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr107131.c" 2


__attribute__((noipa)) unsigned long long
foo (unsigned char o)
{
  unsigned long long t1 = -(long long) (o == 0);
  unsigned long long t2 = -(long long) (t1 > 10439075533421201520ULL);
  unsigned long long t3 = -(long long) (t1 <= t2);
  return t3;
}

int
main ()
{
  if (foo (0) != -1ULL)
    __builtin_abort ();
  return 0;
}
