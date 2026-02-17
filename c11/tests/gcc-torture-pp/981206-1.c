# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/981206-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/981206-1.c" 2


void abort (void);
void exit (int);

static unsigned short x, y;

void foo()
{
  x = 0x345;
  y = 0x567;
}

int main()
{
  foo ();
  if (x != 0x345 || y != 0x567)
    abort ();
  exit (0);
}
