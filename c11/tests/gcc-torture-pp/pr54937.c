# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr54937.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr54937.c" 2

void exit (int);
void abort (void);
int a[1];
void (*terminate_me)(int);

__attribute__((noinline,noclone))
void
t(int c)
{ int i;
  for (i=0;i<c;i++)
    {
      if (i)
       terminate_me(0);
      a[i]=0;
    }
}
int
main(void)
{
  terminate_me = exit;
  t(100);
  abort();
}
