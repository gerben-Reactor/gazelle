# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041201-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041201-1.c" 2


typedef struct { _Complex char a; _Complex char b; } Scc2;

Scc2 s = { 1+2i, 3+4i };

int checkScc2 (Scc2 s)
{
  return s.a != 1+2i || s.b != 3+4i;
}

int main (void)
{
  return checkScc2 (s);
}
