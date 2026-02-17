# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/ifcvt-onecmpl-abs-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/ifcvt-onecmpl-abs-1.c" 2

extern void abort(void);

__attribute__ ((noinline))
int foo(int n)
{
  if (n < 0)
    n = ~n;

  return n;
}

int main(void)
{
  if (foo (-1) != 0)
    abort ();

  return 0;
}
