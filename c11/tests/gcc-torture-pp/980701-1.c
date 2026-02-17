# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980701-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980701-1.c" 2
void abort (void);
void exit (int);

int
ns_name_skip (unsigned char **x, unsigned char *y)
{
  *x = 0;
  return 0;
}

unsigned char a[2];

int dn_skipname(unsigned char *ptr, unsigned char *eom) {
    unsigned char *saveptr = ptr;

    if (ns_name_skip(&ptr, eom) == -1)
         return (-1);
    return (ptr - saveptr);
}

int
main(void)
{
  if (dn_skipname (&a[0], &a[1]) == 0)
    abort ();
  exit (0);
}
