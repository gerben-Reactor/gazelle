# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001027-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001027-1.c" 2
void abort (void);
void exit (int);

int x,*p=&x;

int main()
{
  int i=0;
  x=1;
  p[i]=2;
  if (x != 2)
    abort ();
  exit (0);
}
