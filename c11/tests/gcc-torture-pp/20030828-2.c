# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030828-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030828-2.c" 2
void abort (void);
void exit (int);

struct rtx_def
{
  int code;
};

int
main(void)
{
  int tmp[2];
  struct rtx_def *r, s;
  int *p, *q;





  r = &s;
  r->code = 39;



  p = tmp;
  q = p + 1;
  *q = 0;
  tmp[1] = 39;
  if (*q != 39)
    abort ();
  exit (0);
}
