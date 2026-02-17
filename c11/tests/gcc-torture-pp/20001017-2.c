# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001017-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001017-2.c" 2
void abort (void);

void
fn_4parms (unsigned char a, long *b, long *c, unsigned int *d)
{
  if (*b != 1 || *c != 2 || *d != 3)
    abort ();
}

int
main ()
{
  unsigned char a = 0;
  unsigned long b = 1, c = 2;
  unsigned int d = 3;

  fn_4parms (a, &b, &c, &d);
  return 0;
}
