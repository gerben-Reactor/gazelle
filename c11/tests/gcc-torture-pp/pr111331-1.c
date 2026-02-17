# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111331-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111331-1.c" 2
int a;
int b;
int c(int d, int e, int f) {
  if (d < e)
    return e;
  if (d > f)
    return f;
  return d;
}
int main() {
  int g = -1;
  a = c(b + 30, 29, g + 29);
  volatile int t = a;
  if (t != 28)
    __builtin_abort();
  return 0;
}
