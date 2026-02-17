# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000818-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000818-1.c" 2




void abort (void);
void exit (int);

void *temporary_obstack;

static int input (void);
static int ISALNUM (int ch);
static void obstack_1grow (void **ptr, int ch);

int yylex (void);
int main (void);

int main (void)
{
  int ch = yylex ();

  exit (0);
}

int yylex (void)
{
  int ch;


  for (;;)
    {
      ch = input ();
      if (ISALNUM (ch))
        obstack_1grow (&temporary_obstack, ch);
      else if (ch != '_')
        break;
    }
# 46 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000818-1.c"
  return ch;
}

static int input (void)
{
  return 0;
}

static int ISALNUM (int ch)
{
  return ((ch >= 'A' && ch <= 'Z')
   || (ch >= 'a' && ch <= 'z')
   || (ch >= '0' && ch <= '0'));
}

static void obstack_1grow (void **ptr, int ch)
{
}
