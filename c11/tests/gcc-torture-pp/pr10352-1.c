# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr10352-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr10352-1.c" 2


struct {
  int a:1;
} b;
int *c = (int *)&b, d;
int main() {
  d = c && (b.a = (d < 0) ^ 3);
  if (d != 1)
    __builtin_abort();
  return 0;
}
