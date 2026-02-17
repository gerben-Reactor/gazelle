# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97695.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97695.c" 2
int *a, b, **c = &a, d, e;

int f(int g, int h) { return !h || (g && h == 1) ? 0 : g / h; }

static void *i(int g) {
  while (e < 2)
    if (!f(g, 9)) {
      while (b)
        ;
      return 0;
    }
  return 0;
}

void j() {
  i(1);
  *c = i(d);
}

int main() { j(); return 0; }
