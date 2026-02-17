# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000227-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000227-1.c" 2
void abort(void);

static const unsigned char f[] = "\0\377";
static const unsigned char g[] = "\0ÿ";

int main(void)
{
  if (sizeof f != 3 || sizeof g != 3)
    abort ();
  if (f[0] != g[0])
    abort ();
  if (f[1] != g[1])
    abort ();
  if (f[2] != g[2])
    abort ();
  return 0;
}
