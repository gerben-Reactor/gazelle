# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/widechar-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/widechar-3.c" 2
extern void abort (void);
extern void exit (int);

static int f(char *x)
{
   return __builtin_strlen(x);
}

int foo ()
{
   return f((char*)&L"abcdef"[0]);
}


int
main()
{




  if (foo () != 1)
    abort ();

  exit (0);
}
