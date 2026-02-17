# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961125-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961125-1.c" 2
void abort (void);
void exit (int);

static char *
begfield (int tab, char *ptr, char *lim, int sword, int schar)
{
  if (tab)
    {
      while (ptr < lim && sword--)
 {
   while (ptr < lim && *ptr != tab)
     ++ptr;
   if (ptr < lim)
     ++ptr;
 }
    }
  else
    {
      while (1)
 ;
    }

  if (ptr + schar <= lim)
    ptr += schar;

  return ptr;
}

int
main (void)
{
  char *s = ":ab";
  char *lim = s + 3;
  if (begfield (':', s, lim, 1, 1) != s + 2)
    abort ();
  exit (0);
}
