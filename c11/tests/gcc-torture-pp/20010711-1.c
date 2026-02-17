# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010711-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010711-1.c" 2
void abort (void);
void exit (int);

void foo (int *a) {}

int main ()
{
  int a;
  if (&a == 0)
    abort ();
  else
    {
      foo (&a);
      exit (0);
    }
}
