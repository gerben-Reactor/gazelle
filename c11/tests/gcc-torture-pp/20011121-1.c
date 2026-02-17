# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011121-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011121-1.c" 2
void abort (void);
void exit (int);

struct s
{
  int i[18];
  char f;
  char b[2];
};

struct s s1;

int
main()
{
  struct s s2;
  s2.b[0] = 100;
  __builtin_memcpy(&s2, &s1, ((unsigned int) &((struct s *)0)->b));
  if (s2.b[0] != 100)
    abort();
  exit(0);
}
