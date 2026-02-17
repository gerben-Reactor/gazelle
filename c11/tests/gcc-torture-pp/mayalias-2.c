# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/mayalias-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/mayalias-2.c" 2
typedef struct __attribute__((__may_alias__)) { short x; } test;

int f() {
  int a=10;
  test *p=(test *)&a;
  p->x = 1;
  return a;
}

int main() {
  if (f() == 10)
    __builtin_abort();
  return 0;
}
