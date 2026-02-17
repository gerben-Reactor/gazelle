# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36765.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36765.c" 2
int __attribute__((noinline))
foo(int i)
{
  int *p = __builtin_malloc (4 * sizeof(int));
  *p = 0;
  p[i] = 1;
  return *p;
}
extern void abort (void);
int main()
{
  if (foo(0) != 1)
    abort ();
  return 0;
}
