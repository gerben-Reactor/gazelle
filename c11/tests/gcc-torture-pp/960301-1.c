# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960301-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960301-1.c" 2
void abort (void);
void exit (int);

struct foo {
  unsigned : 12;
  unsigned field : 4;
} foo;
unsigned oldfoo;

int
bar (unsigned k)
{
  oldfoo = foo.field;
  foo.field = k;
  if (k)
    return 1;
  return 2;
}

int
main (void)
{
  if (bar (1U) != 1)
    abort ();
  exit (0);
}
