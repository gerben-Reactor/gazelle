# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030626-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030626-1.c" 2
char buf[10];

extern void abort (void);
extern int sprintf (char*, const char*, ...);

int main()
{
  int l = sprintf (buf, "foo\0bar");
  if (l != 3)
    abort ();
  return 0;
}
