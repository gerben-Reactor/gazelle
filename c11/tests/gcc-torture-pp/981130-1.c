# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/981130-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/981130-1.c" 2

void abort (void);
void exit (int);
struct s { int a; int b;};
struct s s1;
struct s s2 = { 1, 2, };

void
check (a, b)
     int a;
     int b;
{
  if (a == b)
    exit (0);
  else
    abort ();
}

int
main ()
{
  int * p;
  int x;

  s1.a = 9;
  p = & s1.a;
  s1 = s2;
  x = * p;

  check (x, 1);
}
