#include<stdio.h>
#include<inttypes.h>
#include"./ncalc.h"


// gcc main.c -L ./target/release/ -lc_interop_lib -lm
// "-lm" tells linker to include the math library when linking your program.
int main() {
  char* c = "3   \0";
  CResult result =  evaluate(c);
  if( result.bool_result != NULL)
  {
    printf("bool result found: %u\n", *result.bool_result);
  }
  if(result.int_result != NULL)
  {
    printf("int result:%" PRId64"\n", *result.int_result);
  }
  if(result.float_result != NULL)
  {
    printf("found float result: %f\n", *result.float_result);
    float x = 2 + *result.float_result;
    printf("x: %f\n", x);

  }
  free_cresult(result);
  return 0;
}
