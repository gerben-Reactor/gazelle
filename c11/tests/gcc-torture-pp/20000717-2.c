# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-2.c" 2
void abort (void);
void exit (int);

static void
compare (long long foo)
{
  if (foo < 4294967297LL)
    abort();
}
int main(void)
{
  compare (8589934591LL);
  exit (0);
}
