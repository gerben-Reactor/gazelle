# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990222-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990222-1.c" 2
void abort(void);

char line[4] = { '1', '9', '9', '\0' };

int main()
{
  char *ptr = line + 3;

  while ((*--ptr += 1) > '9') *ptr = '0';
  if (line[0] != '2' || line[1] != '0' || line[2] != '0')
    abort();
  return 0;
}
