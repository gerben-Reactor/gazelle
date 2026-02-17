# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36321.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36321.c" 2



extern void abort (void);

extern long unsigned int strlen (const char *);
void foo(char *str)
{
  int len2 = strlen (str);
  char *a = (char *) __builtin_alloca (0);
  char *b = (char *) __builtin_alloca (len2*3);

  if ((int) (a-b) < (len2*3))
    {



      return;
    }
}

static char * volatile argp = "pr36321.x";

int main(int argc, char **argv)
{
  foo (argp);
  return 0;
}
