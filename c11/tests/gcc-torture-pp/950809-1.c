# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950809-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950809-1.c" 2
void abort (void);
void exit (int);

struct S
{
  int *sp, fc, *sc, a[2];
};

void
f (struct S *x)
{
  int *t = x->sc;
  int t1 = t[0];
  int t2 = t[1];
  int t3 = t[2];
  int a0 = x->a[0];
  int a1 = x->a[1];
  t[2] = t1;
  t[0] = a1;
  x->a[1] = a0;
  x->a[0] = t3;
  x->fc = t2;
  x->sp = t;
}

int
main (void)
{
  struct S s;
  static int sc[3] = {2, 3, 4};
  s.sc = sc;
  s.a[0] = 10;
  s.a[1] = 11;
  f (&s);
  if (s.sp[2] != 2)
    abort ();
  exit (0);
}
