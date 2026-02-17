# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020320-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020320-1.c" 2



extern void abort(void);
extern void exit(int);

struct large { int x, y[9]; };

int main()
{
  int fixed;

  fixed = ({ int temp1 = 2; temp1; }) - ({ int temp2 = 1; temp2; });
  if (fixed != 1)
    abort();

  fixed = ({ struct large temp3; temp3.x = 2; temp3; }).x
   - ({ struct large temp4; temp4.x = 1; temp4; }).x;
  if (fixed != 1)
    abort();

  exit(0);
}
