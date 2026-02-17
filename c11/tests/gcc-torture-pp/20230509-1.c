# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230509-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230509-1.c" 2
int __attribute__((noipa)) f(unsigned a, int b)
{
  if (a < 0) __builtin_unreachable();
  if (a > 30) __builtin_unreachable();
  int t = a;
  if (b) t = 100;
  else if (a != 0)
    t = a ;
  else
    t = 1;
  return t;
}


int main(void)
{
  if (f(0, 0) != 1)
    __builtin_abort();
  if (f(1, 0) != 1)
    __builtin_abort();
  if (f(0, 1) != 100)
    __builtin_abort();
  if (f(1, 0) != 1)
    __builtin_abort();
  if (f(30, 0) != 30)
    __builtin_abort();
}
