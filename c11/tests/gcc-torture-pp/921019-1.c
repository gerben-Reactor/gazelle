# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921019-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921019-1.c" 2
void abort (void);
void exit (int);

void *foo[]={(void *)&("X"[0])};

int
main (void)
{
  if (((char*)foo[0])[0] != 'X')
    abort ();
  exit (0);
}
