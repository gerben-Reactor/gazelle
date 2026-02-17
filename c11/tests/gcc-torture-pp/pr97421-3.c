# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-3.c" 2



int a, b, c;
short d;
void e(void) {
  unsigned f = 0;
  for (; f <= 2; f++) {
    int g[1];
    int h = (long)g;
    c = 0;
    for (; c < 10; c++)
      g[0] = a = 0;
    for (; a <= 2; a++)
      b = d;
  }
}
int main(void) {
  e();
  if (a != 3)
    __builtin_abort();
}
