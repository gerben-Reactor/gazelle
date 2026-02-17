# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-1.c" 2

int g(int min_need_stall) __attribute__((__noipa__));
int g(int min_need_stall)
{
  return min_need_stall < 0 ? 1 : ((min_need_stall) < (1) ? (min_need_stall) : (1));
}
int main(void)
{
  for(int i = -100; i <= 100; i++)
    {
      int t = g(i);
      if (t != (i!=0))
        __builtin_abort();
    }
}
