# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22061-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22061-3.c" 2
void abort (void);
void exit (int);

void
bar (int N)
{
  int foo (char a[2][++N]) { N += 4; return sizeof (a[0]); }
  if (foo (0) != 2)
    abort ();
  if (foo (0) != 7)
    abort ();
  if (N != 11)
    abort ();
}

int
main()
{
  bar (1);
  exit (0);
}
