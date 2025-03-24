
int main() {
   int x, y;
   if(x){ y; };
   if(x){ y; } else { y; };
   for(;;){ x + 1; };
   for(int z;;){ x + 2; };
   for(int z=2;;){ x + 3; };
   for(;x;){ x + 4; };
   for(;;x){ x + 5; };
   for(int z; x;){ x + 6; };
   for(int z;;x){ x + 7; };
   for(;x;x){ x + 8; };
   for(int z;x;x){ x + 9; };
   while(x){ x; };
   do{x;} while(x);
   switch(1){
      case 1: continue;
      default: break;
   };
   return;
   return 0;
   label_0:
   goto label_0;
}
