# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010206-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010206-1.c" 2
void abort (void);
void exit (int);

int foo (void)
{
  int i;
# 1 "20010206-1.c"
  if (0) i = 1; else i
# 1 "20010206-1.c"
    = 26;
  return i;
}

int main ()
{
  if (foo () != 26)
    abort ();
  exit (0);
}
