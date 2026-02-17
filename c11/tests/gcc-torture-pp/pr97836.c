# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97836.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97836.c" 2
int a;

int b(int c) { return 0; }

static int *d(int *e) {
  if (a) {
    a = a && b(*e);
  }
  return e;
}

int main() {
  int f;
  if (d(&f) != &f)
    __builtin_abort();
  return 0;
}
