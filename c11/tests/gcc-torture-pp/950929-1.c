# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950929-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950929-1.c" 2
void abort (void);
void exit (int);

int f (char *p) { }

int
main (void)
{
  char c;
  char c2;
  int i = 0;
  char *pc = &c;
  char *pc2 = &c2;
  int *pi = &i;

  *pc2 = 1;
  *pi = 1;
  *pc2 &= *pi;
  f (pc2);
  *pc2 = 1;
  *pc2 &= *pi;
  if (*pc2 != 1)
    abort ();
  exit (0);
}
