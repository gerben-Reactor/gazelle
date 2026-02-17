# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030913-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030913-1.c" 2



void abort (void);
void exit (int);

int g;

void
fn2(int ** q)
{
  *q = &g;
}

void test()
{
  int *p;

  fn2(&p);

  *p=42;
}

int main()
{
  test();
  if (g != 42) abort();
  exit (0);
}
