# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000822-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000822-1.c" 2


void abort (void);

int f0(int (*fn)(int *), int *p)
{
  return (*fn) (p);
}

int f1(void)
{
  int i = 0;

  int f2(int *p)
  {
    i = 1;
    return *p + 1;
  }

  return f0(f2, &i);
}

int main()
{
  if (f1() != 2)
    abort ();

  return 0;
}
