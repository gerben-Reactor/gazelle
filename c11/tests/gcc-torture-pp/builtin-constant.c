# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/builtin-constant.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/builtin-constant.c" 2


void abort (void);
void exit (int);
# 16 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/builtin-constant.c"
void
foo (char *i)
{
}


int
main (void)
{
  int size = sizeof (int);
  foo (__builtin_constant_p(size) ? "1" : "0");
  foo (__builtin_constant_p(size) ? "1" : "0");
  exit (0);
}
