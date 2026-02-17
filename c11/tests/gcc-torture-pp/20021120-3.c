# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021120-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021120-3.c" 2




void abort (void);
void exit (int);







unsigned int foo (char *c, unsigned int x, unsigned int y)
{
  register unsigned int z ;

  __builtin_sprintf (c, "%d", x / y);
  z = x + 1;
  return z / (y + 1);
}

int main ()
{
  char c[16];

  if (foo (c, ~1U, 4) != (~0U / 5))
    abort ();
  exit (0);
}
