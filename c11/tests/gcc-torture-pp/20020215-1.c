# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020215-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020215-1.c" 2
# 14 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020215-1.c"
void abort (void);
void exit (int);

struct s
{
  short i1;
  long i2;
  short i3;
};

struct s foo (struct s s)
{
  s.i2++;
  return s;
}

int main ()
{
  struct s s = foo ((struct s) { 1000, 2000L, 3000 });
  if (s.i1 != 1000 || s.i2 != 2001L || s.i3 != 3000)
    abort ();
  exit (0);
}
