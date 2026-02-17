# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/vrp-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/vrp-4.c" 2
extern void exit (int);
extern void abort ();

void test(int x, int y)
{
 int c;

 if (x == 1) abort();
 if (y == 1) abort();

 c = x / y;

 if (c != 1) abort();
}

int main()
{
 test(2, 2);
 exit (0);
}
