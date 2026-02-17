# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262-2.c" 2





void abort (void);

struct A
{
  int t;
  int i;
};

struct B
{
  int *p;
  float b;
};

float X;

int
foo (struct B b, struct A *q, float *h)
{
  X += *h;
  *(b.p) = 3;
  q->t = 2;
  return *(b.p);
}

int
main(void)
{
  struct A a;
  struct B b;

  b.p = &a.t;
  if (foo (b, &a, &X) == 3)
    abort ();

  return 0;
}
