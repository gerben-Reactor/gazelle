# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980526-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980526-3.c" 2
void abort (void);
void exit (int);

int compare(x, y)
unsigned int x;
unsigned int y;
{
   if (x==y)
     return 0;
   else
     return 1;
}

int
main(void)
{
 unsigned int i, j, k, l;
 i = 5; j = 2; k=0; l=2;
 if (compare(5%(~(unsigned) 2), i%~j)
     || compare(0, k%~l))
    abort();
 else
    exit(0);
}
