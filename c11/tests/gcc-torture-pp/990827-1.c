# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990827-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990827-1.c" 2
void abort (void);
void exit (int);

unsigned test(unsigned one , unsigned bit)
{
    unsigned val= bit & 1;
    unsigned zero= one >> 1;

    val++;
    return zero + ( val>> 1 );
}

int main()
{
  if (test (1,0) != 0)
    abort ();
  if (test (1,1) != 1)
    abort ();
  if (test (1,65535) != 1)
    abort ();
  exit (0);

  return 0;
}
