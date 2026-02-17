# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980424-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980424-1.c" 2
void abort (void);
void exit (int);

int i, a[99];

void f (int one)
{
  if (one != 1)
    abort ();
}

void
g ()
{
  f (a[i & 0x3f]);
}

int
main ()
{
  a[0] = 1;
  i = 0x40;
  g ();
  exit (0);
}
