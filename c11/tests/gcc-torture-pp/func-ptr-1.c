# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/func-ptr-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/func-ptr-1.c" 2
void abort (void);
void exit (int);

static double f (float a);
static double (*fp) (float a);

int
main (void)
{
  fp = f;
  if (fp ((float) 1) != 1.0)
    abort ();
  exit (0);
}

static double
f (float a)
{
  return a;
}
