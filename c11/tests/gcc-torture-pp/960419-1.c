# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960419-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960419-1.c" 2
void abort (void);
void exit (int);

static int i;

void
check(x)
     int x;
{
  if (!x)
    abort();
}

int
main(void)
{
  int *p = &i;

  check(p != (void *)0);
  exit (0);
}
