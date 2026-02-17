# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43629.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43629.c" 2
int flag;
extern void abort (void);
int main()
{
  int x;
  if (flag)
    x = -1;
  else
    x &= 0xff;
  if (x & ~0xff)
    abort ();
  return 0;
}
