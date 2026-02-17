# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24142.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24142.c" 2
void abort (void);

int f (int a, int b)
{
  if (a == 1)
    a = 0;
  if (b == 0)
    a = 1;
  if (a != 0)
    return 0;
  return 1;
}

int main (void)
{
  if (f (1, 1) != 1)
    abort ();
  return 0;
}
