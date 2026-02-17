# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44852.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44852.c" 2
__attribute__ ((__noinline__))
char *sf(char *s, char *s0)
{
  asm ("");
  while (*--s == '9')
    if (s == s0)
      {
 *s = '0';
 break;
      }
  ++*s++;
  return s;
}

int main()
{
  char s[] = "999999";
  char *x = sf (s+2, s);
  if (x != s+1 || __builtin_strcmp (s, "199999") != 0)
    __builtin_abort ();
  return 0;
}
