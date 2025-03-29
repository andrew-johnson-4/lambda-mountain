
typedef int Integer;
typedef int(*f1)();
extern int f2(){}
static int f3;
_Thread_local int f4;
_Thread_local int f5;
const int f8;
volatile int f9;
_Atomic int f10;

//int f11(int *restrict a1, int ** restrict a2, int * restrict * a3) {};
//inline int f12(int *restrict a1, int ** restrict a2, int * restrict * a3) {};
//_Noreturn int f13(int *restrict a1, int ** restrict a2, int * restrict * a3);

int main(){
   auto int f6;
   register int f7;
}
