# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34768-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34768-2.c" 2
int x;

int __attribute__((noinline)) foo (void)
{
  x = -x;
  return 0;
}
int __attribute__((const,noinline)) bar (void)
{
  return 0;
}

int __attribute__((noinline))
test (int c)
{
  int tmp = x;
  int res = (c ? foo : bar) ();
  return tmp + x + res;
}

extern void abort (void);
int main()
{
  x = 1;
  if (test (1) != 0)
    abort ();
  return 0;
}
