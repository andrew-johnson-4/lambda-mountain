
int main() {
   int x, y;
   if(x){ y; };
   if(x){ y; } else { y; };
   for(;;){ x; };
   for(int z;;){ x; };
   for(;x;){ x; };
   for(;;x){ x; };
   for(int z; x;){ x; };
   for(int z;;x){ x; };
   for(;x;x){ x; };
   for(int z;x;x){ x; };
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
