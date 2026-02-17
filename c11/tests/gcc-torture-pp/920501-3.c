# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-3.c" 2


void abort (void);
void exit (int);

int tab[9];
void
execute(oip, ip)
     unsigned short *oip, *ip;
{
  int x = 0;
  int *xp = tab;
base:
  x++;
  if (x == 4)
    {
      *xp = 0;
      return;
    }
  *xp++ = ip - oip;
  goto *(&&base + *ip++);
}

int
main(void)
{
  unsigned short ip[10];
  int i;
  for (i = 0; i < 10; i++)
    ip[i] = 0;
  execute(ip, ip);
  if (tab[0] != 0 || tab[1] != 1 || tab[2] != 2 || tab[3] != 0)
    abort();
  exit(0);
}
