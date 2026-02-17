# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020611-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020611-1.c" 2



void abort (void);
void exit (int);

int p;
int k;
unsigned int n;

void x ()
{
  unsigned int h;

  h = n <= 30;
  if (h)
    p = 1;
  else
    p = 0;

  if (h)
    k = 1;
  else
    k = 0;
}

unsigned int n = 30;

int
main (void)
{
  x ();
  if (p != 1 || k != 1)
    abort ();
  exit (0);
}
