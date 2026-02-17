# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/vrp-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/vrp-1.c" 2

extern void abort ();
extern void exit (int);

int f (int a) {
 if (a != 2) {
  a = -a;
  if (a == 2)
    return 0;
  return 1;
 }
 return 1;
}

int main (int argc, char *argv[]) {
 if (f (-2))
  abort ();
 exit (0);
}
