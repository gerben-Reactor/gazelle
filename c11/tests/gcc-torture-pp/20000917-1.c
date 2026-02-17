# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000917-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000917-1.c" 2



void abort (void);
void exit (int);

typedef struct int3 { int a, b, c; } int3;

int3
one (void)
{
  return (int3) { 1, 1, 1 };
}

int3
zero (void)
{
  return (int3) { 0, 0, 0 };
}

int
main (void)
{
  int3 a;
# 41 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000917-1.c"
  *({ ({ one (); &a; }); }) = zero ();
  if (a.a && a.b && a.c)
    abort ();
  exit (0);
}
