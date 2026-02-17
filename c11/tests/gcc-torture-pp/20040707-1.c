# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040707-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040707-1.c" 2
void exit (int);

struct s { char c1, c2; };
void foo (struct s s)
{
  static struct s s1;
  s1 = s;
}
int main ()
{
  static struct s s2;
  foo (s2);
  exit (0);
}
