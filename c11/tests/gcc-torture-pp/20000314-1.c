# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000314-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000314-1.c" 2
void abort (void);
void exit (int);

int main ()
{
  long winds = 0;

  while (winds != 0)
    {
      if (*(char *) winds)
 break;
    }

  if (winds == 0 || winds != 0 || *(char *) winds)
    exit (0);

  abort ();
}
