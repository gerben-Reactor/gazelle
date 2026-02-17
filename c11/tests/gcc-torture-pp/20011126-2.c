# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011126-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011126-2.c" 2
# 20 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011126-2.c"
static const char *
test (const char *in, char *out)
{
  while (1)
    {
      if (*in == 'a')
 {
   const char *p = in + 1;
   while (*p == 'x')
     ++p;
   if (*p == 'b')
     return p;
   while (in < p)
     *out++ = *in++;
 }
    }
}

int main ()
{
  char out[4];
  test ("aab", out);
  return 0;
}
