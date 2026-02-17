# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21331.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21331.c" 2
void abort (void);

int bar (void) { return -1; }

unsigned long
foo ()
{ unsigned long retval;
  retval = bar ();
  if (retval == -1) return 0;
  return 3; }

int
main (void)
{ if (foo () != 0) abort ();
  return 0; }
