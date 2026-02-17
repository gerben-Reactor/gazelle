# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000314-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000314-3.c" 2
extern void abort (void);
extern void exit (int);

static char arg0[] = "arg0";
static char arg1[] = "arg1";

static void attr_rtx (char *, char *);
static char *attr_string (char *);
static void attr_eq (char *, char *);

static void
attr_rtx (char *varg0, char *varg1)
{
  if (varg0 != arg0)
    abort ();

  if (varg1 != arg1)
    abort ();

  return;
}

static void
attr_eq (name, value)
     char *name, *value;
{
  return attr_rtx (attr_string (name),
     attr_string (value));
}

static char *
attr_string (str)
     char *str;
{
  return str;
}

int main()
{
  attr_eq (arg0, arg1);
  exit (0);
}
