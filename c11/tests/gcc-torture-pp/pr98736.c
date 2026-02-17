# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98736.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98736.c" 2


int a[6];
char b, c;
int main() {
  int d[4] = {0, 0, 0, 0};
  for (c = 0; c <= 5; c++) {
    for (b = 2; b != 0; b++)
      a[c] = 8;
    a[c] = d[3];
  }
  if (a[0] != 0)
    __builtin_abort();
}
