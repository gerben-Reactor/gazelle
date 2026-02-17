# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000706-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000706-4.c" 2
extern void abort(void);
extern void exit(int);

int *c;

void bar(int b)
{
  if (*c != 1 || b != 2)
    abort();
}

void foo(int a, int b)
{
  c = &a;
  bar(b);
}

int main()
{
  foo(1, 2);
  exit(0);
}
