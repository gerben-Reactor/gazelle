# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930123-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930123-1.c" 2
void abort (void);
void exit (int);

void
f(int *x)
{
  *x = 0;
}

int
main(void)
{
  int s, c, x;
  char a[] = "c";

  f(&s);
  a[c = 0] = s == 0 ? (x=1, 'a') : (x=2, 'b');
  if (a[c] != 'a')
    abort();
  exit (0);
}
