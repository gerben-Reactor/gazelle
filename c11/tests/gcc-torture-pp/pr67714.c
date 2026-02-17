# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr67714.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr67714.c" 2
unsigned int b;
int c;

signed char
fn1 ()
{
  signed char d;
  for (int i = 0; i < 1; i++)
    d = -15;
  return d;
}

int
main (void)
{
  for (c = 0; c < 1; c++)
    b = 0;
  char e = fn1 ();
  signed char f = e ^ b;
  volatile int g = (int) f;

  if (g != -15)
    __builtin_abort ();

  return 0;
}
