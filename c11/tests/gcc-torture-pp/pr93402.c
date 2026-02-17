# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93402.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93402.c" 2


struct S { unsigned int a; unsigned long long b; };

__attribute__((noipa)) struct S
foo (unsigned long long x)
{
  struct S ret;
  ret.a = 0;
  ret.b = x * 11111111111ULL + 111111111111ULL;
  return ret;
}

int
main ()
{
  struct S a = foo (1);
  if (a.a != 0 || a.b != 122222222222ULL)
    __builtin_abort ();
  return 0;
}
