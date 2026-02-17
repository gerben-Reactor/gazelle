# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991227-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991227-1.c" 2
void abort (void);
void exit (int);
char* doit(int flag)
{
  return 1 + (flag ? "\0wrong\n" : "\0right\n");
}
int main()
{
  char *result = doit(0);
  if (*result == 'r' && result[1] == 'i')
    exit(0);
  abort();
}
