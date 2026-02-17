# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78617.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78617.c" 2
int a = 0;
int d = 1;
int f = 1;

int fn1() {
  return a || 1 >> a;
}

int fn2(int p1, int p2) {
  return p2 >= 2 ? p1 : p1 >> 1;
}

int fn3(int p1) {
  return d ^ p1;
}

int fn4(int p1, int p2) {
  return fn3(!d > fn2((f = fn1() - 1000) || p2, p1));
}

int main() {
  if (fn4(0, 0) != 1)
    __builtin_abort ();
  return 0;
}
