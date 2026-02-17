# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19606.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19606.c" 2





void abort (void);
void exit (int);

signed char a = -4;

int
foo (void)
{
  return ((unsigned int) (signed int) a) / 2LL;
}

int
bar (void)
{
  return ((unsigned int) (signed int) a) % 5LL;
}

int
main (void)
{
  int r;

  r = foo ();
  if (r != ((unsigned int) (signed int) (signed char) -4) / 2LL)
    abort ();

  r = bar ();
  if (r != ((unsigned int) (signed int) (signed char) -4) % 5LL)
    abort ();

  exit (0);
}
