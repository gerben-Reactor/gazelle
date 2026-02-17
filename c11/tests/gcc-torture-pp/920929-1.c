# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920929-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920929-1.c" 2

void exit (int);
void
f(int n)
{
int i;
double v[n];
for(i=0;i<n;i++)
v[i]=0;
}
int
main(void)
{
f(100);
exit(0);
}
