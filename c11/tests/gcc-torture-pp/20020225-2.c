# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020225-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020225-2.c" 2
void abort (void);
void exit (int);

static int
test(int x)
{
  union
    {
      int i;
      double d;
  } a;
  a.d = 0;
  a.i = 1;
  return x >> a.i;
}

int main(void)
{
  if (test (5) != 2)
    abort ();
  exit (0);
}
