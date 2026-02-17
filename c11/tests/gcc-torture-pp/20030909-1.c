# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030909-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030909-1.c" 2
void abort ();
void exit (int);

void test(int x, int y)
{
  if (x == y)
    abort ();
}

void foo(int x, int y)
{
  if (x == y)
    goto a;
  else
    {
a:;
      if (x == y)
 goto b;
      else
 {
b:;
   if (x != y)
     test (x, y);
 }
    }
}

int main(void)
{
  foo (0, 0);

  exit (0);
}
