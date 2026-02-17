# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000819-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000819-1.c" 2
void abort (void);
void exit (int);

int a[2] = { 2, 0 };

void foo(int *sp, int cnt)
{
  int *p, *top;

  top = sp; sp -= cnt;

  for(p = sp; p <= top; p++)
    if (*p < 2) exit(0);
}

int main()
{
  foo(a + 1, 1);
  abort();
}
