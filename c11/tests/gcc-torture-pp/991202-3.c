# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991202-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991202-3.c" 2
void abort (void);
void exit (int);

unsigned int f (unsigned int a)
{
  return a * 65536 / 8;
}

unsigned int g (unsigned int a)
{
  return a * 65536;
}

unsigned int h (unsigned int a)
{
  return a / 8;
}

int main ()
{
  if (f (65536) != h (g (65536)))
    abort ();
  exit (0);
}
