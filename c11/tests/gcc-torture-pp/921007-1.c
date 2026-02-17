# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921007-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921007-1.c" 2
void abort(void);
void exit(int);
static int strcmp(){return-1;}

int
main(void)
{
if(__builtin_strcmp("X","X\376")>=0)abort();
exit(0);
}
