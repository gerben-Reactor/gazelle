# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-4.c" 2

int a, b = 2, c = 2;
int main() {
  b = ~(1 % (a ^ (b - (1 && c) || c & b)));
  if (b < -1)
    __builtin_abort();
  return 0;
}
