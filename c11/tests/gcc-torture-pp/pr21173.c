# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21173.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21173.c" 2
void abort (void);

char q;
void *a[2];

void foo (char *p)
{
  int i;
  for (i = 0; i < 2; i++)
    a[i] += p - &q;
}

int main (void)
{
  int i;
  foo (&q);
  for (i = 0; i < 2; i ++)
    if (a[i])
      abort ();
  return 0;
}
