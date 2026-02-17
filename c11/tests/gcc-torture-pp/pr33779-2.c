# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr33779-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr33779-2.c" 2
int foo(int i)
{
  return ((int)((unsigned)(i + 1) * 4)) / 4;
}

extern void abort(void);
int main()
{
  if (foo(0x3fffffff) != 0)
    abort ();
  return 0;
}
