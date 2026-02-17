# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-2.c" 2
signed char f() __attribute__((__noipa__));
signed char f() { return 0; }
int main()
{
  int g = f() - 1;
  int e = g < 0 ? 1 : ((g >> (8-2))!=0);
  asm("":"+r"(e));
  if (e != 1)
    __builtin_abort();
}
