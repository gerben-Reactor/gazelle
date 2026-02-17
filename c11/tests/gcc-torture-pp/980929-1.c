# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980929-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980929-1.c" 2
void abort (void);
void exit (int);

void f(int i)
{
  if (i != 1000)
    abort ();
}


int main()
{
  int n=1000;
  int i;

  f(n);
  for(i=0; i<1; ++i) {
    f(n);
    n=666;
    &n;
  }

  exit (0);
}
