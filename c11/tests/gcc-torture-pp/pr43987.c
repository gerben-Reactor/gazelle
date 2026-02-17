# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43987.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43987.c" 2
char B[256 * sizeof(void *)];
typedef void *FILE;
typedef struct globals {
    int c;
    FILE *l;
} __attribute__((may_alias)) T;
void add_input_file(FILE *file)
{
  (*(T*)&B).l[0] = file;
}
extern void abort (void);
int main()
{
  FILE x;
  (*(T*)&B).l = &x;
  add_input_file ((void *)-1);
  if ((*(T*)&B).l[0] != (void *)-1)
    abort ();
  return 0;
}
