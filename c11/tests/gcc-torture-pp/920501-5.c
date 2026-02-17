# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-5.c" 2


void abort (void);
void exit (int);

int
x (int i)
{
  void *j[] = {&&x, &&y, &&z};
  goto *j[i];
 x:return 2;
 y:return 3;
 z:return 5;

}
int
main (void)
{
  if (x (0) != 2 || x (1) != 3 || x (2) != 5)
    abort();
  exit(0);
}
