# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr100453.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr100453.c" 2

struct a {
  int b : 4;
} d;
static int c, e;
static const struct a f;
static void g(const struct a h) {
  for (; c < 1; c++)
    d = h;
  e = h.b;
  c = h.b;
}
int main() {
  g(f);
  return 0;
}
