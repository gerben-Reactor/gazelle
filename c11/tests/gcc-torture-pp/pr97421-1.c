# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-1.c" 2



int a, b, d, e;
int *volatile c = &a;

__attribute__((noinline))
void f(void)
{
  for (int g = 2; g >= 0; g--) {
    d = 0;
    for (b = 0; b <= 2; b++)
      ;
    e = *c;
  }
}

int main(void)
{
  f();
  if (b != 3)
    __builtin_abort();
}
