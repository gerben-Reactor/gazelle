# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111331-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111331-3.c" 2
int a;
int b;

int main() {
  int d = b+30;
  {
    int t;
    t = d < 29 ? 29 : ((d > 28) ? 28 : d);
    a = t;
  }
  volatile int t = a;
  if (a != 28)
    __builtin_abort();
  return 0;
}
