# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020328-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020328-1.c" 2
void abort (void);
void exit (int);

int b = 0;

void func (void) { }

void
testit(int x)
{
  if (x != 20)
    abort ();
}

int
main()

{
  int a = 0;

  if (b)
    func();



  testit ((a + 23) & 0xfffffffc);
  exit (0);
}
