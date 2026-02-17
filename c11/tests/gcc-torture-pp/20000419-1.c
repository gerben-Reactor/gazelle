# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000419-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000419-1.c" 2
void abort (void);
void exit (int);

struct foo { int a, b, c; };

void
brother (int a, int b, int c)
{
  if (a)
    abort ();
}

void
sister (struct foo f, int b, int c)
{
  brother ((f.b == b), b, c);
}

int
main ()
{
  struct foo f = { 7, 8, 9 };
  sister (f, 1, 2);
  exit (0);
}
