# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921117-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921117-1.c" 2
void abort (void);
void exit (int);

struct s {
  char text[11];
  int flag;
} cell;

int
check (struct s p)
{
  if (p.flag != 99)
    return 1;
  return __builtin_strcmp (p.text, "0123456789");
}

int
main (void)
{
  cell.flag = 99;
  __builtin_strcpy (cell.text, "0123456789");

  if (check (cell))
    abort();
  exit (0);
}
