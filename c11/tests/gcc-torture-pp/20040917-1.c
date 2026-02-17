# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040917-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040917-1.c" 2


void abort (void);

static int test_var;





void not_inlinable() __attribute__((noinline));

static void
inlinable ()
{
  test_var = -10;
}

void
not_inlinable ()
{
  inlinable();
}

int
main (void)
{
  test_var = 10;


  not_inlinable ();
  if (test_var == 10)
    abort ();
  return 0;
}
