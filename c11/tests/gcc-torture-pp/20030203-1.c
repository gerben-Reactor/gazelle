# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030203-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030203-1.c" 2
void f(int);
int do_layer3(int single)
{
  int stereo1;

  if(single >= 0)
    stereo1 = 1;
  else
    stereo1 = 2;
  f(single);

  return stereo1;
}

extern void abort ();
int main()
{
  if (do_layer3(-1) != 2)
    abort ();
  return 0;
}

void f(int i) {}
