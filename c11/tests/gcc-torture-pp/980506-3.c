# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980506-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980506-3.c" 2
void abort (void);
void exit (int);

unsigned char lookup_table [257];

static int
build_lookup (pattern)
     unsigned char *pattern;
{
  int m;

  m = __builtin_strlen (pattern) - 1;

  __builtin_memset (lookup_table, ++m, 257);
  return m;
}

int main(argc, argv)
     int argc;
     char **argv;
{
  if (build_lookup ("bind") != 4)
    abort ();
  else
    exit (0);
}
