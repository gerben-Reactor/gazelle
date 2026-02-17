# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010209-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010209-1.c" 2
void abort (void);
void exit (int);

int b;
int foo (void)
{
  int x[b];
  int bar (int t[b])
  {
    int i;
    for (i = 0; i < b; i++)
      t[i] = i + (i > 0 ? t[i-1] : 0);
    return t[b-1];
  }
  return bar (x);
}

int main ()
{
  b = 6;
  if (foo () != 15)
    abort ();
  exit (0);
}
