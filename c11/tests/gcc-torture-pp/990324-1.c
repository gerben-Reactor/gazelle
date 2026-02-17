# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990324-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990324-1.c" 2
void abort (void);
void exit (int);

void f(long i)
{
  if ((signed char)i < 0 || (signed char)i == 0)
    abort ();
  else
    exit (0);
}

int
main(void)
{
  f(0xffffff01);
}
