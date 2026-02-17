# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020413-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020413-1.c" 2
void abort (void);
void exit (int);

void test(long double val, int *eval)
{
  long double tmp = 1.0l;
  int i = 0;

  if (val < 0.0l)
    val = -val;

  if (val >= tmp)
    while (tmp < val)
      {
 tmp *= 2.0l;
 if (i++ >= 10)
   abort ();
      }
  else if (val != 0.0l)
    while (val < tmp)
      {
 tmp /= 2.0l;
 if (i++ >= 10)
   abort ();
      }

  *eval = i;
}

int main(void)
{
  int eval;

  test(3.0, &eval);
  test(3.5, &eval);
  test(4.0, &eval);
  test(5.0, &eval);
  exit (0);
}
