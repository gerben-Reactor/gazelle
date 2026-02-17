# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950221-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950221-1.c" 2


void abort (void);
void exit (int);

struct parsefile
{
  long fd;
  char *buf;
};
struct parsefile basepf;
struct parsefile *parsefile = &basepf;



int filler[0x3000];

int el;

char *
g1 (a, b)
     int a;
     int *b;
{
}

int
g2 (a)
     long a;
{
  if (a != 0xdeadbeefL)
    abort ();
  exit (0);
}

void
f (void)
{
  register char *p, *q;
  register int i;
  register int something;

  if (parsefile->fd == 0L && el)
    {
      const char *rl_cp;
      int len;
      rl_cp = g1 (el, &len);
      __builtin_strcpy (p, rl_cp);
    }
  else
    {
    alabel:
      i = g2 (parsefile->fd);
    }
}

int
main (void)
{
  el = 0;
  parsefile->fd = 0xdeadbeefL;
  f ();
}
