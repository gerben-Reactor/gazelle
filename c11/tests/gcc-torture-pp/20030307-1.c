# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030307-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030307-1.c" 2





extern void abort(void);

int fcntl_lock(int fd, int op, long long offset, long long count, int type);

int vfswrap_lock(char *fsp, int fd, int op, long long offset, long long count, int type)
{
  return fcntl_lock(fd, op, offset, count, type);
}

int fcntl_lock(int fd, int op, long long offset, long long count, int type)
{
  return type;
}

int main(void)
{
  if (vfswrap_lock (0, 1, 2, 3, 4, 5) != 5)
    abort();

  return 0;
}
