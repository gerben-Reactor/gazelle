# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr46316.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr46316.c" 2
extern void abort (void);

long long __attribute__((noinline,noclone))
foo (long long t)
{
  while (t > -4)
    t -= 2;

  return t;
}

int main(void)
{
  if (foo (0) != -4)
    abort ();
  return 0;
}
