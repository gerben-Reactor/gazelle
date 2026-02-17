# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr38212.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr38212.c" 2
int __attribute__((noinline))
foo (int *__restrict p, int i)
{
  int *__restrict q;
  int *__restrict r;
  int v, w;
  q = p + 1;
  r = q - i;
  v = *r;
  *p = 1;
  w = *r;
  return v + w;
}
extern void abort (void);
int main()
{
  int i = 0;
  if (foo (&i, 1) != 1)
    abort ();
  return 0;
}
