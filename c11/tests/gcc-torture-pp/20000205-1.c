# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000205-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000205-1.c" 2
void abort (void);
void exit (int);

static int f (int a)
{
  if (a == 0)
    return 0;
  do
    if (a & 128)
      return 1;
  while (f (0));
  return 0;
}

int main(void)
{
  if (f (~128))
    abort ();
  exit (0);
}
