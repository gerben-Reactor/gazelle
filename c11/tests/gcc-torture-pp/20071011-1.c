# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071011-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071011-1.c" 2
extern void abort(void);
void foo(int *p)
{
  int x;
  int y;
  x = *p;
  *p = 0;
  y = *p;
  if (x != y)
    return;
  abort ();
}

int main()
{
  int a = 1;
  foo(&a);
  return 0;
}
