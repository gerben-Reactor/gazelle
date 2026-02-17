# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090814-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090814-1.c" 2
int __attribute__((noinline))
bar (int *a)
{
  return *a;
}
int i;
int __attribute__((noinline))
foo (int (*a)[2])
{
  return bar (&(*a)[i]);
}

extern void abort (void);
int a[2];
int main()
{
  a[0] = -1;
  a[1] = 42;
  i = 1;
  if (foo (&a) != 42)
    abort ();
  return 0;
}
