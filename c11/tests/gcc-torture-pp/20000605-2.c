# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000605-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000605-2.c" 2
void abort (void);
void exit (int);

struct F { int i; };

void f1(struct F *x, struct F *y)
{
  int timeout = 0;
  for (; ((const struct F*)x)->i < y->i ; x->i++)
    if (++timeout > 5)
      abort ();
}

int
main(void)
{
  struct F x, y;
  x.i = 0;
  y.i = 1;
  f1 (&x, &y);
  exit (0);
}
