# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010422-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010422-1.c" 2
void abort (void);
void exit (int);

unsigned int foo(unsigned int x)
{
  if (x < 5)
    x = 4;
  else
    x = 8;
  return x;
}

int main(void)
{
  if (foo (8) != 8)
    abort ();
  exit (0);
}
