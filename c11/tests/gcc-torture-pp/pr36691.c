# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36691.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36691.c" 2
unsigned char g_5;

void func_1 (void)
{
  for (g_5 = 9; g_5 >= 4; g_5 -= 5)
    ;
}

extern void abort (void);
int main (void)
{
  func_1 ();
  if (g_5 != 0)
    abort ();
  return 0;
}
