# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960326-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960326-1.c" 2
void abort (void);
void exit (int);

struct s
{
  int a;
  int b;
  short c;
  int d[3];
};

struct s s = { .b = 3, .d = {2,0,0} };

int
main (void)
{
  if (s.b != 3)
    abort ();
  exit (0);
}
