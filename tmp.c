#include <regex.h>
#include <sys/wait.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
typedef struct LM_Buffer LM_Buffer;
typedef struct LM_SmartString LM_SmartString;
typedef struct LM_S LM_S;
typedef struct LM_TupleListTupleTypeTypeTupleU64String LM_TupleListTupleTypeTypeTupleU64String;
typedef struct LM_TupleSmartStringU64 LM_TupleSmartStringU64;
typedef struct LM_TupleSS LM_TupleSS;
typedef struct LM_TupleListTypeListType LM_TupleListTypeListType;
typedef struct LM_TupleTContextTContext LM_TupleTContextTContext;
typedef struct LM_TupleU64String LM_TupleU64String;
typedef struct LM_TupleTypeType LM_TupleTypeType;
typedef struct LM_ListString LM_ListString;
typedef struct LM_ListType LM_ListType;
typedef struct LM_ListTupleTypeType LM_ListTupleTypeType;
typedef struct LM_HashtableEqListTupleTypeTypeTupleU64String LM_HashtableEqListTupleTypeTypeTupleU64String;
typedef struct LM_HashtableEqSmartStringU64 LM_HashtableEqSmartStringU64;
typedef struct LM_Ord LM_Ord;
typedef struct LM_UmbraShortLong LM_UmbraShortLong;
typedef struct LM_Umbra LM_Umbra;
typedef struct LM_Type LM_Type;
typedef struct LM_SourceLocation LM_SourceLocation;
typedef struct LM_Token LM_Token;
typedef struct LM_AST LM_AST;
typedef struct LM_TContext LM_TContext;
struct LM_Buffer{
	long field_0;
	union {
struct {unsigned long field_1;unsigned long field_2;char* field_3;};
	};
};
struct LM_SmartString{
	long field_0;
	union {
struct {char* field_1;char* field_2;char* field_3;char* field_4;};
	};
};
struct LM_S{
	long field_0;
	union {
struct {char* field_1;};
struct {LM_S* field_1001;LM_S* field_1002;};
struct {char* field_2001;};
	};
};
struct LM_TupleSmartStringU64{
	long field_0;
	union {
struct {LM_SmartString field_1;unsigned long field_2;};
	};
};
struct LM_TupleSS{
	long field_0;
	union {
struct {LM_S field_1;LM_S field_2;};
	};
};
struct LM_TupleU64String{
	long field_0;
	union {
struct {unsigned long field_1;char* field_2;};
	};
};
struct LM_ListString{
	long field_0;
	union {
struct {LM_ListString* field_1;char* field_2;};
	};
};
struct LM_HashtableEqListTupleTypeTypeTupleU64String{
	long field_0;
	union {
struct {LM_TupleListTupleTypeTypeTupleU64String* field_1001;unsigned long field_1002;unsigned long field_1003;};
	};
};
struct LM_HashtableEqSmartStringU64{
	long field_0;
	union {
struct {LM_TupleSmartStringU64* field_1001;unsigned long field_1002;unsigned long field_1003;};
	};
};
struct LM_Ord{
	long field_0;
};
struct LM_UmbraShortLong{
	long field_0;
	union {
struct {char* field_1;char field_2[4];};
struct {char field_1001[12];};
	};
};
struct LM_Umbra{
	long field_0;
	union {
struct {LM_UmbraShortLong field_1;unsigned int field_2;};
	};
};
struct LM_Type{
	long field_0;
	union {
struct {LM_Type* field_1;LM_Type* field_2;};
struct {LM_ListType* field_1001;char* field_1002;};
struct {char* field_2001;};
	};
};
struct LM_SourceLocation{
	long field_0;
	union {
struct {unsigned long field_1;unsigned long field_2;char* field_3;};
	};
};
struct LM_Token{
	long field_0;
	union {
struct {LM_SourceLocation field_1;unsigned long field_2;char* field_3;};
	};
};
struct LM_AST{
	long field_0;
	union {
struct {LM_AST* field_1;};
struct {LM_AST* field_1001;LM_AST* field_1002;};
struct {LM_AST* field_2001;LM_Token field_2002;};
struct {LM_AST* field_3001;LM_AST* field_3002;};
struct {LM_Type field_4001;};
struct {LM_Type field_5001;LM_AST* field_5002;LM_AST* field_5003;};
struct {LM_Token field_6001;char* field_6002;};
struct {LM_Token field_7001;char* field_7002;};
struct {LM_AST* field_8001;LM_AST* field_8002;unsigned long field_8003;};
	};
};
struct LM_TContext{
	long field_0;
	union {
struct {LM_AST field_1;LM_Type field_2;char* field_3;LM_TContext* field_4;};
	};
};
struct LM_ListType{
	long field_0;
	union {
struct {LM_ListType* field_1;LM_Type field_2;};
	};
};
struct LM_TupleTypeType{
	long field_0;
	union {
struct {LM_Type field_1;LM_Type field_2;};
	};
};
struct LM_TupleTContextTContext{
	long field_0;
	union {
struct {LM_TContext field_1;LM_TContext field_2;};
	};
};
struct LM_TupleListTypeListType{
	long field_0;
	union {
struct {LM_ListType field_1;LM_ListType field_2;};
	};
};
struct LM_ListTupleTypeType{
	long field_0;
	union {
struct {LM_ListTupleTypeType* field_1;LM_TupleTypeType field_2;};
	};
};
struct LM_TupleListTupleTypeTypeTupleU64String{
	long field_0;
	union {
struct {LM_ListTupleTypeType field_1;LM_TupleU64String field_2;};
	};
};
unsigned long true_CL_U64;
unsigned long false_CL_U64;
#line 4 "PLATFORM/C/LIB/i8.lm"
void print_CL_ArrowNilI8(signed char uuid__000000000001ea90);
#line 4 "PLATFORM/C/LIB/u16.lm"
void print_CL_ArrowNilU16(unsigned short uuid__000000000001ea91);
#line 4 "PLATFORM/C/LIB/i16.lm"
void print_CL_ArrowNilI16(signed short uuid__000000000001ea92);
#line 4 "PLATFORM/C/LIB/u32.lm"
void print_CL_ArrowNilU32(unsigned int uuid__000000000001ea93);
#line 4 "PLATFORM/C/LIB/i32.lm"
void print_CL_ArrowNilI32(signed int uuid__000000000001ea94);
#line 4 "PLATFORM/C/LIB/u64.lm"
void print_CL_ArrowNilConsU64IO_CL__CL_File(FILE* uuid__000000000001ea95,unsigned long uuid__000000000001ea96);
#line 8 "PLATFORM/C/LIB/u64.lm"
unsigned long max_CL_ArrowU64ConsU64U64(unsigned long uuid__000000000001ea97,unsigned long uuid__000000000001ea98);
#line 12 "PLATFORM/C/LIB/u64.lm"
unsigned long min_CL_ArrowU64ConsU64U64(unsigned long uuid__000000000001ea99,unsigned long uuid__000000000001ea9a);
#line 16 "PLATFORM/C/LIB/u64.lm"
char* to_SB_string_CL_ArrowStringU64(unsigned long uuid__000000000001ea9b);
#line 33 "PLATFORM/C/LIB/u64.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringU64(unsigned long uuid__000000000001ea9f);
#line 37 "PLATFORM/C/LIB/u64.lm"
unsigned long deep_SB_hash_CL_ArrowU64U64(unsigned long uuid__000000000001eaa0);
#line 39 "PLATFORM/C/LIB/u64.lm"
unsigned long to_SB_u64_CL_ArrowU64String(char* uuid__000000000001eaa1);
#line 62 "PLATFORM/C/LIB/u64.lm"
char* to_SB_hex_CL_ArrowStringU64(unsigned long uuid__000000000001eaae);
signed long minimum_SB_I64_CL_I64;
#line 6 "PLATFORM/C/LIB/i64.lm"
void print_CL_ArrowNilI64(signed long uuid__000000000001eac2);
#line 14 "PLATFORM/C/LIB/i64.lm"
char* to_SB_string_CL_ArrowStringI64(signed long uuid__000000000001eac3);
#line 36 "PLATFORM/C/LIB/i64.lm"
signed long to_SB_i64_CL_ArrowI64String(char* uuid__000000000001eac8);
#line 49 "PLATFORM/C/LIB/i64.lm"
signed long max_CL_ArrowI64ConsI64I64(signed long uuid__000000000001eacb,signed long uuid__000000000001eacc);
#line 53 "PLATFORM/C/LIB/i64.lm"
signed long min_CL_ArrowI64ConsI64I64(signed long uuid__000000000001eacd,signed long uuid__000000000001eace);
#line 4 "PLATFORM/C/LIB/f64.lm"
void print_CL_ArrowNilF64(double uuid__000000000001eacf);
#line 8 "PLATFORM/C/LIB/f64.lm"
double max_CL_ArrowF64ConsF64F64(double uuid__000000000001ead0,double uuid__000000000001ead1);
#line 12 "PLATFORM/C/LIB/f64.lm"
double min_CL_ArrowF64ConsF64F64(double uuid__000000000001ead2,double uuid__000000000001ead3);
#line 20 "PLATFORM/C/LIB/u8.lsts"
void print_CL_ArrowNilU8(char uuid__000000000001ead4);
#line 22 "PLATFORM/C/LIB/u8.lsts"
char* clone_SB_rope_CL_ArrowStringU8(char uuid__000000000001ead5);
#line 29 "PLATFORM/C/LIB/u8.lsts"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringU8(char uuid__000000000001ead7);
#line 20 "PLATFORM/C/LIB/u32.lsts"
LM_Ord cmp_CL_ArrowOrdConsU32U32(unsigned int uuid__000000000001ead8,unsigned int uuid__000000000001ead9);
#line 26 "PLATFORM/C/LIB/u64.lsts"
LM_Ord cmp_CL_ArrowOrdConsU64U64(unsigned long uuid__000000000001eada,unsigned long uuid__000000000001eadb);
#line 5 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer new_SB_buffer_CL_ArrowBufferU64(unsigned long uuid__000000000001eadc);
#line 10 "PLATFORM/C/LIB/buffer.lm"
unsigned long _DT_calculate_SB_extension_SB_size_CL_ArrowU64U64(unsigned long uuid__000000000001eade);
#line 18 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer _DT_extend_CL_ArrowBufferConsU64Buffer(LM_Buffer uuid__000000000001eae0,unsigned long uuid__000000000001eae1);
#line 29 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer _DT_write_CL_ArrowBufferConsU8Buffer(LM_Buffer uuid__000000000001eae4,char uuid__000000000001eae5);
#line 4 "PLATFORM/C/LIB/string.lm"
char head_SB_string_CL_ArrowU8String(char* uuid__000000000001eae7);
#line 8 "PLATFORM/C/LIB/string.lm"
char* tail_SB_string_CL_ArrowStringString(char* uuid__000000000001eae8);
#line 12 "PLATFORM/C/LIB/string.lm"
char* clone_SB_rope_CL_ArrowStringS(LM_S uuid__000000000001eae9);
#line 19 "PLATFORM/C/LIB/string.lm"
LM_Buffer clone_SB_rope_SB_impl_CL_ArrowBufferConsSBuffer(LM_Buffer uuid__000000000001eaeb,LM_S uuid__000000000001eaec);
#line 40 "PLATFORM/C/LIB/string.lm"
unsigned long deep_SB_hash_CL_ArrowU64String(char* uuid__000000000001eaf8);
#line 55 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_length_CL_ArrowU64String(char* uuid__000000000001eafb);
#line 63 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsStringString(char* uuid__000000000001eafd,char* uuid__000000000001eafe);
#line 84 "PLATFORM/C/LIB/string.lm"
char* _DT_remove_SB_prefix_CL_ArrowStringConsStringString(char* uuid__000000000001eb02,char* uuid__000000000001eb03);
#line 94 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_has_SB_suffix_CL_ArrowU64ConsStringString(char* uuid__000000000001eb04,char* uuid__000000000001eb05);
#line 105 "PLATFORM/C/LIB/string.lm"
char* _DT_remove_SB_suffix_CL_ArrowStringConsStringString(char* uuid__000000000001eb07,char* uuid__000000000001eb08);
#line 121 "PLATFORM/C/LIB/string.lm"
char* _DT_replace_CL_ArrowStringConsStringConsStringString(char* uuid__000000000001eb0a,char* uuid__000000000001eb0b,char* uuid__000000000001eb0c);
#line 135 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_contains_CL_ArrowU64ConsStringString(char* uuid__000000000001eb0e,char* uuid__000000000001eb0f);
#line 144 "PLATFORM/C/LIB/string.lm"
char* _AD__CL_ArrowStringConsStringString(char* uuid__000000000001eb11,char* uuid__000000000001eb12);
#line 148 "PLATFORM/C/LIB/string.lm"
void print_CL_ArrowNilString(char* uuid__000000000001eb13);
#line 155 "PLATFORM/C/LIB/string.lm"
unsigned long non_SB_zero_CL_ArrowU64String(char* uuid__000000000001eb14);
#line 2 "PLATFORM/C/LIB/string.lsts"
LM_Ord cmp_CL_ArrowOrdConsStringString(char* uuid__000000000001eb15,char* uuid__000000000001eb16);
#line 9 "PLATFORM/C/LIB/string.lsts"
void print_CL_ArrowNilConsStringIO_CL__CL_File(FILE* uuid__000000000001eb18,char* uuid__000000000001eb19);
#line 16 "PLATFORM/C/LIB/string.lsts"
unsigned long _DT_is_SB_digit_CL_ArrowU64String(char* uuid__000000000001eb1a);
#line 4 "PLATFORM/C/LIB/smart-string.lm"
unsigned long non_SB_zero_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb1c);
#line 8 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString intern_CL_ArrowSmartStringString(char* uuid__000000000001eb1d);
#line 13 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString intern_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb1f);
#line 17 "PLATFORM/C/LIB/smart-string.lm"
char* untern_CL_ArrowStringSmartString(LM_SmartString uuid__000000000001eb20);
#line 29 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_length_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb23);
#line 33 "PLATFORM/C/LIB/smart-string.lm"
void print_CL_ArrowNilSmartString(LM_SmartString uuid__000000000001eb24);
#line 41 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsU64ConsU64SmartString(LM_SmartString uuid__000000000001eb26,unsigned long uuid__000000000001eb27,unsigned long uuid__000000000001eb28);
#line 44 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsU64ConsI64SmartString(LM_SmartString uuid__000000000001eb29,signed long uuid__000000000001eb2a,unsigned long uuid__000000000001eb2b);
#line 47 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsI64ConsU64SmartString(LM_SmartString uuid__000000000001eb2c,unsigned long uuid__000000000001eb2d,signed long uuid__000000000001eb2e);
#line 51 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(LM_SmartString uuid__000000000001eb2f,signed long uuid__000000000001eb30,signed long uuid__000000000001eb31);
#line 79 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString tail_SB_string_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb34);
#line 83 "PLATFORM/C/LIB/smart-string.lm"
char _LB__RB__CL_ArrowU8ConsI64SmartString(LM_SmartString uuid__000000000001eb35,signed long uuid__000000000001eb36);
#line 90 "PLATFORM/C/LIB/smart-string.lm"
char _LB__RB__CL_ArrowU8ConsU64SmartString(LM_SmartString uuid__000000000001eb37,unsigned long uuid__000000000001eb38);
#line 101 "PLATFORM/C/LIB/smart-string.lm"
char head_SB_string_CL_ArrowU8SmartString(LM_SmartString uuid__000000000001eb3a);
#line 105 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_has_SB_suffix_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3b,LM_SmartString uuid__000000000001eb3c);
#line 113 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _DT_remove_SB_suffix_CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3d,LM_SmartString uuid__000000000001eb3e);
#line 119 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3f,LM_SmartString uuid__000000000001eb40);
#line 127 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _DT_remove_SB_prefix_CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb41,LM_SmartString uuid__000000000001eb42);
#line 133 "PLATFORM/C/LIB/smart-string.lm"
char* _DT_replace_CL_ArrowStringConsSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb43,LM_SmartString uuid__000000000001eb44,LM_SmartString uuid__000000000001eb45);
#line 148 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _AD__CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb47,LM_SmartString uuid__000000000001eb48);
#line 165 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb4d);
#line 166 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringString(char* uuid__000000000001eb4e);
#line 168 "PLATFORM/C/LIB/smart-string.lm"
unsigned long deep_SB_hash_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb4f);
#line 2 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsSmartStringString(char* uuid__000000000001eb53,LM_SmartString uuid__000000000001eb54);
#line 5 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb55,LM_SmartString uuid__000000000001eb56);
#line 6 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb57,LM_SmartString uuid__000000000001eb58);
#line 7 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb59,LM_SmartString uuid__000000000001eb5a);
#line 8 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5b,LM_SmartString uuid__000000000001eb5c);
#line 9 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5d,LM_SmartString uuid__000000000001eb5e);
#line 10 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5f,LM_SmartString uuid__000000000001eb60);
#line 12 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsStringSmartString(LM_SmartString uuid__000000000001eb61,char* uuid__000000000001eb62);
#line 15 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb63,char* uuid__000000000001eb64);
#line 16 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb65,char* uuid__000000000001eb66);
#line 17 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb67,char* uuid__000000000001eb68);
#line 18 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb69,char* uuid__000000000001eb6a);
#line 19 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb6b,char* uuid__000000000001eb6c);
#line 20 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb6d,char* uuid__000000000001eb6e);
#line 22 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsSmartStringSmartString(LM_SmartString uuid__000000000001eb6f,LM_SmartString uuid__000000000001eb70);
#line 47 "PLATFORM/C/LIB/smart-string.lsts"
void print_CL_ArrowNilConsSmartStringIO_CL__CL_File(FILE* uuid__000000000001eb74,LM_SmartString uuid__000000000001eb75);
#line 3 "PLATFORM/C/LIB/s.lsts"
unsigned long non_SB_zero_CL_ArrowU64S(LM_S uuid__000000000001eb77);
LM_SmartString uuid__000000000001eb7c;
LM_SmartString uuid__000000000001eb85;
LM_SmartString uuid__000000000001eb87;
LM_SmartString uuid__000000000001eb89;
LM_SmartString uuid__000000000001eb8e;
LM_SmartString uuid__000000000001eb90;
#line 5 "PLATFORM/C/LIB/s.lsts"
void print_CL_ArrowNilConsSIO_CL__CL_File(FILE* uuid__000000000001eb78,LM_S uuid__000000000001eb79);
#line 18 "PLATFORM/C/LIB/s.lsts"
LM_S _AD__CL_ArrowSConsSS(LM_S uuid__000000000001eb92,LM_S uuid__000000000001eb93);
#line 26 "PLATFORM/C/LIB/s.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSS(LM_S uuid__000000000001eb94,LM_S uuid__000000000001eb95);
#line 38 "PLATFORM/C/LIB/s.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsSS(LM_S uuid__000000000001ebc7,LM_S uuid__000000000001ebc8);
#line 134 "PLATFORM/C/LIB/list.lm"
char* _DT_join_CL_ArrowStringConsStringListString(LM_ListString uuid__000000000001ebc9,char* uuid__000000000001ebca);
#line 37 "PLATFORM/C/LIB/io.lm"
void exit_CL_ArrowNilU64(unsigned long uuid__000000000001ebcc);
#line 39 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverConsStringString(char* uuid__000000000001ebcd,char* uuid__000000000001ebce);
#line 44 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverSmartString(LM_SmartString uuid__000000000001ebcf);
#line 48 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverString(char* uuid__000000000001ebd0);
#line 53 "PLATFORM/C/LIB/io.lm"
char* read_SB_file_CL_ArrowStringString(char* uuid__000000000001ebd1);
#line 71 "PLATFORM/C/LIB/io.lm"
void write_SB_file_CL_ArrowNilConsStringString(char* uuid__000000000001ebd6,char* uuid__000000000001ebd7);
#line 77 "PLATFORM/C/LIB/io.lm"
unsigned long file_SB_exists_CL_ArrowU64String(char* uuid__000000000001ebd9);
#line 2 "PLATFORM/C/LIB/io.lsts"
char* _DT_file_SB_extension_CL_ArrowStringString(char* uuid__000000000001ebdc);
#line 15 "PLATFORM/C/LIB/regex.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsRegexSmartString(LM_SmartString uuid__000000000001ebdd,regex_t uuid__000000000001ebde);
#line 26 "PLATFORM/C/LIB/regex.lm"
LM_SmartString _DT_remove_SB_prefix_CL_ArrowSmartStringConsRegexSmartString(LM_SmartString uuid__000000000001ebe0,regex_t uuid__000000000001ebe1);
#line 9 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe4,LM_Ord uuid__000000000001ebe5);
#line 10 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe6,LM_Ord uuid__000000000001ebe7);
#line 11 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _LT__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe8,LM_Ord uuid__000000000001ebe9);
#line 12 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebea,LM_Ord uuid__000000000001ebeb);
#line 13 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _GT__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebec,LM_Ord uuid__000000000001ebed);
#line 14 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebee,LM_Ord uuid__000000000001ebef);
#line 23 "PLATFORM/C/LIB/cmp.lsts"
LM_Ord _AM__AM__CL_ArrowOrdConsOrdOrd(LM_Ord uuid__000000000001ebf0,LM_Ord uuid__000000000001ebf1);
#line 40 "PLATFORM/C/LIB/umbra.lsts"
unsigned long _DT_length_CL_ArrowU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebf2);
#line 46 "PLATFORM/C/LIB/umbra.lsts"
char _LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebf3,unsigned long uuid__000000000001ebf4);
#line 60 "PLATFORM/C/LIB/umbra.lsts"
void set_LB__RB__CL_ArrowNilConsU8ConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebfd,unsigned long uuid__000000000001ebfe,char uuid__000000000001ebff);
#line 79 "PLATFORM/C/LIB/umbra.lsts"
char* addr_CL_ArrowArray_QM_U8Array_QM_Umbra(LM_Umbra* uuid__000000000001ec08);
#line 92 "PLATFORM/C/LIB/umbra.lsts"
LM_Umbra view_SB_len_CL_ArrowUmbraConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ec10,unsigned long uuid__000000000001ec11);
#line 108 "PLATFORM/C/LIB/umbra.lsts"
void print_CL_ArrowNilArray_QM_Umbra(LM_Umbra* uuid__000000000001ec14);
#line 117 "PLATFORM/C/LIB/umbra.lsts"
unsigned long short_SB_prefix_SB_matches_CL_ArrowU64ConsArray_QM_UmbraArray_QM_Umbra(LM_Umbra* uuid__000000000001ec17,LM_Umbra* uuid__000000000001ec18);
#line 125 "PLATFORM/C/LIB/umbra.lsts"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsArray_QM_UmbraArray_QM_Umbra(LM_Umbra* uuid__000000000001ec19,LM_Umbra* uuid__000000000001ec1a);
#line 186 "PLATFORM/C/LIB/umbra.lsts"
unsigned long deep_SB_hash_CL_ArrowU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ec1b);
#line 202 "PLATFORM/C/LIB/umbra.lsts"
LM_Umbra new_SB_umbra_CL_ArrowUmbraU64(unsigned long uuid__000000000001ec1f);
#line 6 "SRC/types-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64Type(LM_Type uuid__000000000001ec22);
#line 2 "SRC/type-print.lm"
void print_CL_ArrowNilType(LM_Type uuid__000000000001ec23);
#line 26 "SRC/type-print.lm"
void print_CL_ArrowNilListType(LM_ListType uuid__000000000001ec35);
#line 2 "SRC/t.lsts"
LM_Type t1_CL_ArrowTypeString(char* uuid__000000000001ec40);
#line 3 "SRC/t.lsts"
LM_Type t2_CL_ArrowTypeConsTypeString(char* uuid__000000000001ec42,LM_Type uuid__000000000001ec43);
#line 4 "SRC/t.lsts"
LM_Type t3_CL_ArrowTypeConsTypeConsTypeString(char* uuid__000000000001ec45,LM_Type uuid__000000000001ec46,LM_Type uuid__000000000001ec47);
#line 5 "SRC/t.lsts"
LM_Type tv_CL_ArrowTypeString(char* uuid__000000000001ec49);
#line 6 "SRC/t.lsts"
LM_Type _AM__AM__CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001ec4a,LM_Type uuid__000000000001ec4b);
#line 7 "SRC/t.lsts"
LM_Type _BR__BR__CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001ec4c,LM_Type uuid__000000000001ec4d);
#line 2 "SRC/p.lsts"
LM_Type _DT_p_CL_ArrowTypeConsU64Type(LM_Type uuid__000000000001ec4e,unsigned long uuid__000000000001ec4f);
#line 8 "SRC/p.lsts"
LM_Type _DT_p1_CL_ArrowTypeType(LM_Type uuid__000000000001ec54);
#line 9 "SRC/p.lsts"
LM_Type _DT_p2_CL_ArrowTypeType(LM_Type uuid__000000000001ec55);
#line 2 "SRC/range.lsts"
LM_Type _DT_range_CL_ArrowTypeType(LM_Type uuid__000000000001ec56);
#line 2 "SRC/domain.lsts"
LM_Type _DT_domain_CL_ArrowTypeType(LM_Type uuid__000000000001ec6b);
#line 2 "SRC/arity.lsts"
unsigned long _DT_arity_CL_ArrowU64Type(LM_Type uuid__000000000001ec80);
#line 2 "SRC/slot.lsts"
LM_Type _DT_slot_CL_ArrowTypeConsStringType(LM_Type uuid__000000000001ec85,char* uuid__000000000001ec86);
#line 2 "SRC/is-t.lsts"
unsigned long _DT_is_SB_t_CL_ArrowU64ConsStringType(LM_Type uuid__000000000001ec90,char* uuid__000000000001ec91);
#line 2 "SRC/is-open.lsts"
unsigned long _DT_is_SB_open_CL_ArrowU64Type(LM_Type uuid__000000000001ec9a);
#line 2 "SRC/is-arrow.lsts"
unsigned long _DT_is_SB_arrow_CL_ArrowU64Type(LM_Type uuid__000000000001ecba);
#line 2 "SRC/has-class.lsts"
unsigned long _DT_has_SB_class_CL_ArrowU64Type(LM_Type uuid__000000000001ecca);
#line 3 "SRC/without-tag.lsts"
LM_Type _DT_without_SB_tag_CL_ArrowTypeType(LM_Type uuid__000000000001ecd1);
#line 14 "SRC/without-tag.lsts"
LM_ListType _DT_without_SB_tag_CL_ArrowListTypeListType(LM_ListType uuid__000000000001ece3);
#line 16 "SRC/ast-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64AST(LM_AST uuid__000000000001ece9);
#line 19 "SRC/ast-definitions.lsts"
LM_AST App_CL_ArrowASTConsArray_QM_ASTArray_QM_AST(LM_AST* uuid__000000000001ecea,LM_AST* uuid__000000000001eceb);
#line 2 "SRC/ast-location.lsts"
LM_SourceLocation _DT_location_CL_ArrowSourceLocationAST(LM_AST uuid__000000000001ecec);
#line 2 "SRC/mk-location.lsts"
LM_SourceLocation mk_SB_location_CL_ArrowSourceLocationNil();
#line 2 "SRC/mk-token.lsts"
LM_Token mk_SB_token_CL_ArrowTokenString(char* uuid__000000000001ed08);
#line 6 "SRC/mk-token.lsts"
LM_Token mk_SB_token_CL_ArrowTokenSmartString(LM_SmartString uuid__000000000001ed09);
#line 2 "SRC/is-seq.lsts"
unsigned long _DT_is_SB_seq_CL_ArrowU64AST(LM_AST uuid__000000000001ed0a);
#line 2 "SRC/non-zero.lsts"
unsigned long non_SB_zero_CL_ArrowU64SourceLocation(LM_SourceLocation uuid__000000000001ed0d);
#line 6 "SRC/non-zero.lsts"
LM_SourceLocation _BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation(LM_SourceLocation uuid__000000000001ed0e,LM_SourceLocation uuid__000000000001ed0f);
unsigned long uuid_SB_counter_CL_U64;
#line 4 "SRC/uuid.lsts"
char* uuid_CL_ArrowStringNil();
#line 12 "SRC/uuid.lsts"
unsigned long iuid_CL_ArrowU64Nil();
LM_HashtableEqSmartStringU64 smart_SB_token_SB_path_SB_index_CL_HashtableEqSmartStringU64;
LM_SmartString uuid__000000000001ed14;
#line 4 "SRC/smart-token-location.lsts"
LM_SourceLocation _DT_location_CL_ArrowSourceLocationSmartString(LM_SmartString uuid__000000000001ed12);
#line 4 "SRC/inference-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64TContext(LM_TContext uuid__000000000001ed19);
#line 2 "SRC/can-unify.lm"
unsigned long can_SB_unify_CL_ArrowU64ConsListTypeListType(LM_ListType uuid__000000000001ed1a,LM_ListType uuid__000000000001ed1b);
#line 19 "SRC/can-unify.lm"
unsigned long can_SB_unify_CL_ArrowU64ConsTypeType(LM_Type uuid__000000000001ed2c,LM_Type uuid__000000000001ed2d);
#line 2 "SRC/unify.lm"
LM_TContext unify_CL_ArrowTContextConsTypeType(LM_Type uuid__000000000001edf9,LM_Type uuid__000000000001edfa);
#line 10 "SRC/unify.lm"
LM_TContext unify_SB_inner_CL_ArrowTContextConsTypeType(LM_Type uuid__000000000001edfc,LM_Type uuid__000000000001edfd);
#line 153 "SRC/unify.lm"
LM_TContext unify_CL_ArrowTContextConsListTypeListType(LM_ListType uuid__000000000001ef10,LM_ListType uuid__000000000001ef11);
#line 2 "SRC/tctx-substitute.lm"
LM_ListType substitute_CL_ArrowListTypeConsListTypeTContext(LM_TContext uuid__000000000001ef22,LM_ListType uuid__000000000001ef23);
#line 16 "SRC/tctx-substitute.lm"
LM_Type substitute_CL_ArrowTypeConsTypeTContext(LM_TContext uuid__000000000001ef2a,LM_Type uuid__000000000001ef2b);
#line 3 "SRC/tctx-union.lm"
LM_TContext union_CL_ArrowTContextConsTContextTContext(LM_TContext uuid__000000000001ef3f,LM_TContext uuid__000000000001ef40);
#line 4 "SRC/tctx-and.lm"
LM_TContext and_CL_ArrowTContextConsTContextTContext(LM_TContext uuid__000000000001ef4a,LM_TContext uuid__000000000001ef4b);
LM_HashtableEqListTupleTypeTypeTupleU64String quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String;
#line 4 "SRC/quick-prop.lsts"
LM_TupleU64String _DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(LM_Type uuid__000000000001ef65);
#line 21 "SRC/quick-prop.lsts"
void add_SB_quick_SB_prop_CL_ArrowNilConsTypeConsTypeType(LM_Type uuid__000000000001effb,LM_Type uuid__000000000001effc,LM_Type uuid__000000000001effd);
#line 28 "SRC/quick-prop.lsts"
LM_Type enrich_SB_quick_SB_prop_CL_ArrowTypeType(LM_Type uuid__000000000001f01e);
#line 32 "SRC/quick-prop.lsts"
LM_Type enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001f01f,LM_Type uuid__000000000001f020);
LM_Type t_SB_A_CL_Type;
LM_Type t_SB_B_CL_Type;
LM_Type t_SB_C_CL_Type;
LM_Type ti1_CL_Type;
LM_Type ti2_CL_Type;
#line 2 "PLATFORM/C/LIB/print.lsts"
void print_CL_ArrowNilU64(unsigned long uuid__000000000001f04b);
#line 22 "PLATFORM/C/LIB/array.lm"
LM_S* close_CL_ArrowArray_QM_SS(LM_S uuid__000000000001f04c);
#line 3 "PLATFORM/C/LIB/sized.lm"
unsigned long hash_CL_ArrowU64U64(unsigned long uuid__000000000001f04e);
#line 16 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsStringString(char* uuid__000000000001f051,char* uuid__000000000001f052);
#line 16 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001f053,LM_SmartString uuid__000000000001f054);
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001f055,LM_SmartString uuid__000000000001f056);
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListString(LM_ListString uuid__000000000001f05b);
#line 31 "PLATFORM/C/LIB/list.lm"
LM_ListString tail_CL_ArrowListStringListString(LM_ListString uuid__000000000001f05d);
#line 14 "PLATFORM/C/LIB/list.lm"
char* head_CL_ArrowStringListString(LM_ListString uuid__000000000001f064);
#line 22 "PLATFORM/C/LIB/array.lm"
LM_ListType* close_CL_ArrowArray_QM_ListTypeListType(LM_ListType uuid__000000000001f067);
#line 9 "PLATFORM/C/LIB/list.lm"
LM_ListType list_CL__CL_cons_CL_ArrowListTypeConsListTypeType(LM_Type uuid__000000000001f069,LM_ListType uuid__000000000001f06a);
#line 22 "PLATFORM/C/LIB/array.lm"
LM_Type* close_CL_ArrowArray_QM_TypeType(LM_Type uuid__000000000001f06e);
#line 91 "PLATFORM/C/LIB/list.lm"
LM_Type _DT_nth_CL_ArrowTypeConsTypeConsU64ListType(LM_ListType uuid__000000000001f070,unsigned long uuid__000000000001f071,LM_Type uuid__000000000001f072);
#line 31 "PLATFORM/C/LIB/list.lm"
LM_ListType tail_CL_ArrowListTypeListType(LM_ListType uuid__000000000001f07b);
#line 14 "PLATFORM/C/LIB/list.lm"
LM_Type head_CL_ArrowTypeListType(LM_ListType uuid__000000000001f082);
#line 23 "PLATFORM/C/LIB/list.lm"
unsigned long _DT_has_SB_head_CL_ArrowU64ListType(LM_ListType uuid__000000000001f085);
#line 52 "PLATFORM/C/LIB/list.lm"
unsigned long _DT_length_CL_ArrowU64ListType(LM_ListType uuid__000000000001f088);
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListType(LM_ListType uuid__000000000001f08e);
#line 4 "PLATFORM/C/LIB/list.lm"
LM_ListType cons_CL_ArrowListTypeConsListTypeType(LM_Type uuid__000000000001f090,LM_ListType uuid__000000000001f091);
#line 5 "PLATFORM/C/LIB/hashtable.lsts"
LM_SmartString _DT_lookup_CL_ArrowSmartStringConsSmartStringConsU64HashtableEqSmartStringU64(LM_HashtableEqSmartStringU64 uuid__000000000001f095,unsigned long uuid__000000000001f096,LM_SmartString uuid__000000000001f097);
#line 30 "PLATFORM/C/LIB/sized.lm"
unsigned long mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleSmartStringU64(LM_TupleSmartStringU64 uuid__000000000001f0a3);
#line 22 "PLATFORM/C/LIB/array.lm"
LM_TContext* close_CL_ArrowArray_QM_TContextTContext(LM_TContext uuid__000000000001f0a6);
#line 40 "PLATFORM/C/LIB/hashtable.lsts"
LM_HashtableEqListTupleTypeTypeTupleU64String _DT_bind_CL_ArrowHashtableEqListTupleTypeTypeTupleU64StringConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0a8,LM_TupleU64String uuid__000000000001f0af,LM_ListTupleTypeType uuid__000000000001f0b0);
#line 68 "PLATFORM/C/LIB/hashtable.lsts"
unsigned long _DT_bind_SB_eq_CL_ArrowU64ConsListTupleTypeTypeConsTupleU64StringConsU64Array_QM_TupleListTupleTypeTypeTupleU64String(LM_TupleListTupleTypeTypeTupleU64String* uuid__000000000001f0ba,unsigned long uuid__000000000001f0bb,LM_TupleU64String uuid__000000000001f0c2,LM_ListTupleTypeType uuid__000000000001f0c3);
#line 30 "PLATFORM/C/LIB/sized.lm"
unsigned long mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(LM_TupleListTupleTypeTypeTupleU64String uuid__000000000001f0c7);
#line 16 "PLATFORM/C/LIB/tuple.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String(LM_TupleU64String uuid__000000000001f0d0,LM_TupleU64String uuid__000000000001f0d7);
#line 8 "PLATFORM/C/LIB/tuple.lsts"
unsigned long deep_SB_hash_CL_ArrowU64TupleU64String(LM_TupleU64String uuid__000000000001f0de);
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsHashtableEqListTupleTypeTypeTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0df,LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0e0);
#line 4 "PLATFORM/C/LIB/list.lm"
LM_ListTupleTypeType cons_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeTupleTypeType(LM_TupleTypeType uuid__000000000001f0eb,LM_ListTupleTypeType uuid__000000000001f0ec);
#line 22 "PLATFORM/C/LIB/array.lm"
LM_ListTupleTypeType* close_CL_ArrowArray_QM_ListTupleTypeTypeListTupleTypeType(LM_ListTupleTypeType uuid__000000000001f0ed);
#line 5 "PLATFORM/C/LIB/hashtable.lsts"
LM_ListTupleTypeType _DT_lookup_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0ef,LM_TupleU64String uuid__000000000001f0f6,LM_ListTupleTypeType uuid__000000000001f0f7);
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsTypeType(LM_Type uuid__000000000001f0fd,LM_Type uuid__000000000001f0fe);
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListTupleTypeType(LM_ListTupleTypeType uuid__000000000001f103);
#line 64 "PLATFORM/C/LIB/list.lm"
LM_ListType _DT_reverse_CL_ArrowListTypeListType(LM_ListType uuid__000000000001f104);
#line 4 "PLATFORM/C/LIB/i8.lm"
void print_CL_ArrowNilI8(signed char uuid__000000000001ea90){(print_CL_ArrowNilI64(((signed long)(uuid__000000000001ea90))));}
#line 4 "PLATFORM/C/LIB/u16.lm"
void print_CL_ArrowNilU16(unsigned short uuid__000000000001ea91){(print_CL_ArrowNilU64(((unsigned long)(uuid__000000000001ea91))));}
#line 4 "PLATFORM/C/LIB/i16.lm"
void print_CL_ArrowNilI16(signed short uuid__000000000001ea92){(print_CL_ArrowNilI64(((signed long)(uuid__000000000001ea92))));}
#line 4 "PLATFORM/C/LIB/u32.lm"
void print_CL_ArrowNilU32(unsigned int uuid__000000000001ea93){(print_CL_ArrowNilU64(((unsigned long)(uuid__000000000001ea93))));}
#line 4 "PLATFORM/C/LIB/i32.lm"
void print_CL_ArrowNilI32(signed int uuid__000000000001ea94){(print_CL_ArrowNilI64(((signed long)(uuid__000000000001ea94))));}
#line 4 "PLATFORM/C/LIB/u64.lm"
void print_CL_ArrowNilConsU64IO_CL__CL_File(FILE* uuid__000000000001ea95,unsigned long uuid__000000000001ea96){({(fprintf(uuid__000000000001ea95,"%lu",uuid__000000000001ea96));
({});
});}
#line 8 "PLATFORM/C/LIB/u64.lm"
unsigned long max_CL_ArrowU64ConsU64U64(unsigned long uuid__000000000001ea97,unsigned long uuid__000000000001ea98){return ((uuid__000000000001ea97>=uuid__000000000001ea98)?uuid__000000000001ea97:uuid__000000000001ea98);}
#line 12 "PLATFORM/C/LIB/u64.lm"
unsigned long min_CL_ArrowU64ConsU64U64(unsigned long uuid__000000000001ea99,unsigned long uuid__000000000001ea9a){return ((uuid__000000000001ea99<=uuid__000000000001ea9a)?uuid__000000000001ea99:uuid__000000000001ea9a);}
#line 16 "PLATFORM/C/LIB/u64.lm"
char* to_SB_string_CL_ArrowStringU64(unsigned long uuid__000000000001ea9b){LM_S uuid__000000000001ea9c;
char uuid__000000000001ea9d;
char uuid__000000000001ea9e;
return ({({({({({uuid__000000000001ea9c=({LM_S rvalue={3};rvalue;});({});})
;
({uuid__000000000001ea9d=(0);({});})
;
});
({while((!(uuid__000000000001ea9b==(0)))){((void)(({({({uuid__000000000001ea9e=((char)(((uuid__000000000001ea9b%(10))+(48))));({});})
;
({uuid__000000000001ea9b=(uuid__000000000001ea9b/(10));({});});
});
({uuid__000000000001ea9c=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001ea9c));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8(uuid__000000000001ea9e));rvalue;})));rvalue;});({});});
})));};});
});
((non_SB_zero_CL_ArrowU64S(uuid__000000000001ea9c))?({}):({uuid__000000000001ea9c=({LM_S rvalue={.field_0=2};rvalue.field_2001="0";rvalue;});({});}));
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001ea9c));
});}
#line 33 "PLATFORM/C/LIB/u64.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringU64(unsigned long uuid__000000000001ea9f){return (intern_CL_ArrowSmartStringString((to_SB_string_CL_ArrowStringU64(uuid__000000000001ea9f))));}
#line 37 "PLATFORM/C/LIB/u64.lm"
unsigned long deep_SB_hash_CL_ArrowU64U64(unsigned long uuid__000000000001eaa0){return (hash_CL_ArrowU64U64(uuid__000000000001eaa0));}
#line 39 "PLATFORM/C/LIB/u64.lm"
unsigned long to_SB_u64_CL_ArrowU64String(char* uuid__000000000001eaa1){unsigned long uuid__000000000001eaa2;
char uuid__000000000001eaa3;
char uuid__000000000001eaa4;
char uuid__000000000001eaa5;
char uuid__000000000001eaa6;
char uuid__000000000001eaa7;
char uuid__000000000001eaa8;
char uuid__000000000001eaa9;
char uuid__000000000001eaaa;
char uuid__000000000001eaab;
char uuid__000000000001eaac;
char uuid__000000000001eaad;
return ({({({uuid__000000000001eaa2=(0);({});})
;
({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eaa1))){((void)(({({({uuid__000000000001eaa2=(uuid__000000000001eaa2*(10));({});});
({({uuid__000000000001eaa3=(head_SB_string_CL_ArrowU8String(uuid__000000000001eaa1));({});})
;
(({({uuid__000000000001eaa4=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa4==(48));
})?({}):(({({uuid__000000000001eaa5=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa5==(49));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(1));({});}):(({({uuid__000000000001eaa6=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa6==(50));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(2));({});}):(({({uuid__000000000001eaa7=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa7==(51));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(3));({});}):(({({uuid__000000000001eaa8=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa8==(52));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(4));({});}):(({({uuid__000000000001eaa9=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaa9==(53));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(5));({});}):(({({uuid__000000000001eaaa=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaaa==(54));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(6));({});}):(({({uuid__000000000001eaab=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaab==(55));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(7));({});}):(({({uuid__000000000001eaac=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaac==(56));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(8));({});}):(({({uuid__000000000001eaad=uuid__000000000001eaa3;({});})
;
(uuid__000000000001eaad==(57));
})?({uuid__000000000001eaa2=(uuid__000000000001eaa2+(9));({});}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/u64.lm Line: 43 Column: 8")))))))))))));
});
});
({uuid__000000000001eaa1=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eaa1));({});});
})));};});
});
uuid__000000000001eaa2;
});}
#line 62 "PLATFORM/C/LIB/u64.lm"
char* to_SB_hex_CL_ArrowStringU64(unsigned long uuid__000000000001eaae){LM_S uuid__000000000001eaaf;
unsigned long uuid__000000000001eab0;
unsigned long uuid__000000000001eab1;
unsigned long uuid__000000000001eab2;
unsigned long uuid__000000000001eab3;
unsigned long uuid__000000000001eab4;
unsigned long uuid__000000000001eab5;
unsigned long uuid__000000000001eab6;
unsigned long uuid__000000000001eab7;
unsigned long uuid__000000000001eab8;
unsigned long uuid__000000000001eab9;
unsigned long uuid__000000000001eaba;
unsigned long uuid__000000000001eabb;
unsigned long uuid__000000000001eabc;
unsigned long uuid__000000000001eabd;
unsigned long uuid__000000000001eabe;
unsigned long uuid__000000000001eabf;
unsigned long uuid__000000000001eac0;
unsigned long uuid__000000000001eac1;
return ({({({({uuid__000000000001eaaf=({LM_S rvalue={3};rvalue;});({});})
;
({uuid__000000000001eab0=(16);({});})
;
});
({while((uuid__000000000001eab0>(0))){((void)(({({({({uuid__000000000001eab1=(uuid__000000000001eaae%(16));({});})
;
(({({uuid__000000000001eab2=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab2==(0));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="0";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab3=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab3==(1));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="1";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab4=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab4==(2));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="2";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab5=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab5==(3));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="3";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab6=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab6==(4));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="4";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab7=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab7==(5));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="5";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab8=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab8==(6));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="6";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eab9=uuid__000000000001eab1;({});})
;
(uuid__000000000001eab9==(7));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="7";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eaba=uuid__000000000001eab1;({});})
;
(uuid__000000000001eaba==(8));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="8";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eabb=uuid__000000000001eab1;({});})
;
(uuid__000000000001eabb==(9));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="9";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eabc=uuid__000000000001eab1;({});})
;
(uuid__000000000001eabc==(10));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="a";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eabd=uuid__000000000001eab1;({});})
;
(uuid__000000000001eabd==(11));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="b";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eabe=uuid__000000000001eab1;({});})
;
(uuid__000000000001eabe==(12));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="c";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eabf=uuid__000000000001eab1;({});})
;
(uuid__000000000001eabf==(13));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="d";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eac0=uuid__000000000001eab1;({});})
;
(uuid__000000000001eac0==(14));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="e";rvalue;})));rvalue;});({});}):(({({uuid__000000000001eac1=uuid__000000000001eab1;({});})
;
(uuid__000000000001eac1==(15));
})?({uuid__000000000001eaaf=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eaaf));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="f";rvalue;})));rvalue;});({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/u64.lm Line: 66 Column: 8"))))))))))))))))));
});
({uuid__000000000001eaae=(uuid__000000000001eaae/(16));({});});
});
({uuid__000000000001eab0=(uuid__000000000001eab0-(1));({});});
})));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001eaaf));
});}
#line 6 "PLATFORM/C/LIB/i64.lm"
void print_CL_ArrowNilI64(signed long uuid__000000000001eac2){({((uuid__000000000001eac2<(0))?({(print_CL_ArrowNilString("-"));
({uuid__000000000001eac2=((0)-uuid__000000000001eac2);({});});
}):({}));
(print_CL_ArrowNilU64(((unsigned long)(uuid__000000000001eac2))));
});}
#line 14 "PLATFORM/C/LIB/i64.lm"
char* to_SB_string_CL_ArrowStringI64(signed long uuid__000000000001eac3){LM_S uuid__000000000001eac4;
LM_S uuid__000000000001eac5;
char uuid__000000000001eac6;
char uuid__000000000001eac7;
return ({({({({({({({uuid__000000000001eac4=({LM_S rvalue={3};rvalue;});({});})
;
({uuid__000000000001eac5=({LM_S rvalue={3};rvalue;});({});})
;
});
((uuid__000000000001eac3<(0))?({({uuid__000000000001eac4=({LM_S rvalue={.field_0=2};rvalue.field_2001="-";rvalue;});({});});
({uuid__000000000001eac3=((0)-uuid__000000000001eac3);({});});
}):({}));
});
({uuid__000000000001eac6=(0);({});})
;
});
({while((!(uuid__000000000001eac3==(0)))){((void)(({({({uuid__000000000001eac7=((char)(((((unsigned long)(uuid__000000000001eac3))%(10))+(48))));({});})
;
({uuid__000000000001eac3=(uuid__000000000001eac3/(10));({});});
});
({uuid__000000000001eac5=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eac5));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8(uuid__000000000001eac7));rvalue;})));rvalue;});({});});
})));};});
});
((non_SB_zero_CL_ArrowU64S(uuid__000000000001eac5))?({}):({uuid__000000000001eac5=({LM_S rvalue={.field_0=2};rvalue.field_2001="0";rvalue;});({});}));
});
(clone_SB_rope_CL_ArrowStringS(({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eac5));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(uuid__000000000001eac4));rvalue;})));
});}
#line 36 "PLATFORM/C/LIB/i64.lm"
signed long to_SB_i64_CL_ArrowI64String(char* uuid__000000000001eac8){char uuid__000000000001eac9;
signed long uuid__000000000001eaca;
return ({({({({({uuid__000000000001eac9=(0);({});})
;
(((head_SB_string_CL_ArrowU8String(uuid__000000000001eac8))==(45))?({({uuid__000000000001eac9=(1);({});});
({uuid__000000000001eac8=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eac8));({});});
}):({}));
});
({uuid__000000000001eaca=((signed long)((to_SB_u64_CL_ArrowU64String(uuid__000000000001eac8))));({});})
;
});
((uuid__000000000001eac9==(1))?({uuid__000000000001eaca=((0)-uuid__000000000001eaca);({});}):({}));
});
uuid__000000000001eaca;
});}
#line 49 "PLATFORM/C/LIB/i64.lm"
signed long max_CL_ArrowI64ConsI64I64(signed long uuid__000000000001eacb,signed long uuid__000000000001eacc){return ((uuid__000000000001eacb>=uuid__000000000001eacc)?uuid__000000000001eacb:uuid__000000000001eacc);}
#line 53 "PLATFORM/C/LIB/i64.lm"
signed long min_CL_ArrowI64ConsI64I64(signed long uuid__000000000001eacd,signed long uuid__000000000001eace){return ((uuid__000000000001eacd<=uuid__000000000001eace)?uuid__000000000001eacd:uuid__000000000001eace);}
#line 4 "PLATFORM/C/LIB/f64.lm"
void print_CL_ArrowNilF64(double uuid__000000000001eacf){({(printf("%lf",uuid__000000000001eacf));
({});
});}
#line 8 "PLATFORM/C/LIB/f64.lm"
double max_CL_ArrowF64ConsF64F64(double uuid__000000000001ead0,double uuid__000000000001ead1){return ((uuid__000000000001ead0>=uuid__000000000001ead1)?uuid__000000000001ead0:uuid__000000000001ead1);}
#line 12 "PLATFORM/C/LIB/f64.lm"
double min_CL_ArrowF64ConsF64F64(double uuid__000000000001ead2,double uuid__000000000001ead3){return ((uuid__000000000001ead2<=uuid__000000000001ead3)?uuid__000000000001ead2:uuid__000000000001ead3);}
#line 20 "PLATFORM/C/LIB/u8.lsts"
void print_CL_ArrowNilU8(char uuid__000000000001ead4){(print_CL_ArrowNilU64(((unsigned long)(uuid__000000000001ead4))));}
#line 22 "PLATFORM/C/LIB/u8.lsts"
char* clone_SB_rope_CL_ArrowStringU8(char uuid__000000000001ead5){char* uuid__000000000001ead6;
return ({({({({uuid__000000000001ead6=((char*)((malloc((2)))));({});})
;
(uuid__000000000001ead6[(0)]=uuid__000000000001ead5);
});
(uuid__000000000001ead6[(1)]=(0));
});
((char*)(uuid__000000000001ead6));
});}
#line 29 "PLATFORM/C/LIB/u8.lsts"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringU8(char uuid__000000000001ead7){return (intern_CL_ArrowSmartStringString((to_SB_string_CL_ArrowStringU64(((unsigned long)(uuid__000000000001ead7))))));}
#line 20 "PLATFORM/C/LIB/u32.lsts"
LM_Ord cmp_CL_ArrowOrdConsU32U32(unsigned int uuid__000000000001ead8,unsigned int uuid__000000000001ead9){return ((uuid__000000000001ead8==uuid__000000000001ead9)?({LM_Ord rvalue={1};rvalue;}):((uuid__000000000001ead8<uuid__000000000001ead9)?({LM_Ord rvalue={0};rvalue;}):({LM_Ord rvalue={2};rvalue;})));}
#line 26 "PLATFORM/C/LIB/u64.lsts"
LM_Ord cmp_CL_ArrowOrdConsU64U64(unsigned long uuid__000000000001eada,unsigned long uuid__000000000001eadb){return ((uuid__000000000001eada==uuid__000000000001eadb)?({LM_Ord rvalue={1};rvalue;}):((uuid__000000000001eada<uuid__000000000001eadb)?({LM_Ord rvalue={0};rvalue;}):({LM_Ord rvalue={2};rvalue;})));}
#line 5 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer new_SB_buffer_CL_ArrowBufferU64(unsigned long uuid__000000000001eadc){void* uuid__000000000001eadd;
return ({({uuid__000000000001eadd=(malloc(uuid__000000000001eadc));({});})
;
({LM_Buffer rvalue={.field_0=0};rvalue.field_1=uuid__000000000001eadc;rvalue.field_2=(0);rvalue.field_3=((char*)(uuid__000000000001eadd));rvalue;});
});}
#line 10 "PLATFORM/C/LIB/buffer.lm"
unsigned long _DT_calculate_SB_extension_SB_size_CL_ArrowU64U64(unsigned long uuid__000000000001eade){unsigned long uuid__000000000001eadf;
return ({({({uuid__000000000001eadf=(1024);({});})
;
({while((uuid__000000000001eadf<uuid__000000000001eade)){((void)(({uuid__000000000001eadf=(uuid__000000000001eadf*(4));({});})));};});
});
uuid__000000000001eadf;
});}
#line 18 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer _DT_extend_CL_ArrowBufferConsU64Buffer(LM_Buffer uuid__000000000001eae0,unsigned long uuid__000000000001eae1){unsigned long uuid__000000000001eae2;
void* uuid__000000000001eae3;
return ({((((uuid__000000000001eae0.field_2)+uuid__000000000001eae1)<(uuid__000000000001eae0.field_1))?({uuid__000000000001eae0=({LM_Buffer rvalue={.field_0=0};rvalue.field_1=(uuid__000000000001eae0.field_1);rvalue.field_2=((uuid__000000000001eae0.field_2)+uuid__000000000001eae1);rvalue.field_3=(uuid__000000000001eae0.field_3);rvalue;});({});}):({({({uuid__000000000001eae2=(_DT_calculate_SB_extension_SB_size_CL_ArrowU64U64(((uuid__000000000001eae0.field_2)+uuid__000000000001eae1)));({});})
;
({uuid__000000000001eae3=(realloc((uuid__000000000001eae0.field_3),uuid__000000000001eae2));({});})
;
});
({uuid__000000000001eae0=({LM_Buffer rvalue={.field_0=0};rvalue.field_1=uuid__000000000001eae2;rvalue.field_2=((uuid__000000000001eae0.field_2)+uuid__000000000001eae1);rvalue.field_3=((char*)(uuid__000000000001eae3));rvalue;});({});});
}));
uuid__000000000001eae0;
});}
#line 29 "PLATFORM/C/LIB/buffer.lm"
LM_Buffer _DT_write_CL_ArrowBufferConsU8Buffer(LM_Buffer uuid__000000000001eae4,char uuid__000000000001eae5){LM_Buffer uuid__000000000001eae6;
return ({({({uuid__000000000001eae6=(_DT_extend_CL_ArrowBufferConsU64Buffer(uuid__000000000001eae4,(1)));({});})
;
(((char*)((uuid__000000000001eae6.field_3)))[(uuid__000000000001eae4.field_2)]=uuid__000000000001eae5);
});
uuid__000000000001eae6;
});}
#line 4 "PLATFORM/C/LIB/string.lm"
char head_SB_string_CL_ArrowU8String(char* uuid__000000000001eae7){return (((char*)(uuid__000000000001eae7))[(0)]);}
#line 8 "PLATFORM/C/LIB/string.lm"
char* tail_SB_string_CL_ArrowStringString(char* uuid__000000000001eae8){return (((char*)(uuid__000000000001eae8))+(1));}
#line 12 "PLATFORM/C/LIB/string.lm"
char* clone_SB_rope_CL_ArrowStringS(LM_S uuid__000000000001eae9){LM_Buffer uuid__000000000001eaea;
return ({({({({uuid__000000000001eaea=(new_SB_buffer_CL_ArrowBufferU64((64)));({});})
;
({uuid__000000000001eaea=(clone_SB_rope_SB_impl_CL_ArrowBufferConsSBuffer(uuid__000000000001eaea,uuid__000000000001eae9));({});});
});
({uuid__000000000001eaea=(_DT_write_CL_ArrowBufferConsU8Buffer(uuid__000000000001eaea,(0)));({});});
});
((char*)((uuid__000000000001eaea.field_3)));
});}
#line 19 "PLATFORM/C/LIB/string.lm"
LM_Buffer clone_SB_rope_SB_impl_CL_ArrowBufferConsSBuffer(LM_Buffer uuid__000000000001eaeb,LM_S uuid__000000000001eaec){LM_S uuid__000000000001eaed;
LM_S uuid__000000000001eaee;
LM_S uuid__000000000001eaef;
char uuid__000000000001eaf0;
LM_S uuid__000000000001eaf1;
LM_S uuid__000000000001eaf2;
LM_S uuid__000000000001eaf3;
char uuid__000000000001eaf4;
char* uuid__000000000001eaf5;
unsigned long uuid__000000000001eaf6;
char uuid__000000000001eaf7;
return ({({({uuid__000000000001eaed=uuid__000000000001eaec;({});})
;
(({({uuid__000000000001eaee=uuid__000000000001eaed;({});})
;
((uuid__000000000001eaee.field_0)==(3));
})?({}):(({({({({uuid__000000000001eaef=uuid__000000000001eaed;({});})
;
({uuid__000000000001eaf0=(0);({});})
;
});
(((uuid__000000000001eaef.field_0)==(1))?(({({uuid__000000000001eaf1=(*(((LM_S)(uuid__000000000001eaef)).field_1001));({});})
;
1;
})?(({({uuid__000000000001eaf2=(*(((LM_S)(uuid__000000000001eaef)).field_1002));({});})
;
1;
})?({uuid__000000000001eaf0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eaf0==(1));
})?({({uuid__000000000001eaeb=(clone_SB_rope_SB_impl_CL_ArrowBufferConsSBuffer(uuid__000000000001eaeb,uuid__000000000001eaf2));({});});
({uuid__000000000001eaeb=(clone_SB_rope_SB_impl_CL_ArrowBufferConsSBuffer(uuid__000000000001eaeb,uuid__000000000001eaf1));({});});
}):(({({({({uuid__000000000001eaf3=uuid__000000000001eaed;({});})
;
({uuid__000000000001eaf4=(0);({});})
;
});
(((uuid__000000000001eaf3.field_0)==(2))?(({({uuid__000000000001eaf5=(((LM_S)(uuid__000000000001eaf3)).field_2001);({});})
;
1;
})?({uuid__000000000001eaf4=(1);({});}):({})):({}));
});
(uuid__000000000001eaf4==(1));
})?({({({uuid__000000000001eaf6=(0);({});})
;
({uuid__000000000001eaf7=(((char*)(uuid__000000000001eaf5))[uuid__000000000001eaf6]);({});})
;
});
({while((uuid__000000000001eaf7!=(0))){((void)(({({({uuid__000000000001eaeb=(_DT_write_CL_ArrowBufferConsU8Buffer(uuid__000000000001eaeb,uuid__000000000001eaf7));({});});
({uuid__000000000001eaf6=(uuid__000000000001eaf6+(1));({});});
});
({uuid__000000000001eaf7=(((char*)(uuid__000000000001eaf5))[uuid__000000000001eaf6]);({});});
})));};});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/string.lm Line: 20 Column: 5")))));
});
uuid__000000000001eaeb;
});}
#line 40 "PLATFORM/C/LIB/string.lm"
unsigned long deep_SB_hash_CL_ArrowU64String(char* uuid__000000000001eaf8){unsigned long uuid__000000000001eaf9;
unsigned long uuid__000000000001eafa;
return ({({({({({({({uuid__000000000001eaf9=(0);({});})
;
({uuid__000000000001eafa=(0);({});})
;
});
({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eaf8))){((void)(({({({({uuid__000000000001eafa=(uuid__000000000001eafa+((unsigned long)((head_SB_string_CL_ArrowU8String(uuid__000000000001eaf8)))));({});});
({uuid__000000000001eaf8=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eaf8));({});});
});
({uuid__000000000001eafa=(uuid__000000000001eafa+(uuid__000000000001eafa<<(10)));({});});
});
({uuid__000000000001eafa=(uuid__000000000001eafa^(uuid__000000000001eafa>>(6)));({});});
})));};});
});
({uuid__000000000001eafa=(uuid__000000000001eafa+(uuid__000000000001eafa<<(3)));({});});
});
({uuid__000000000001eafa=(uuid__000000000001eafa^(uuid__000000000001eafa>>(11)));({});});
});
({uuid__000000000001eafa=(uuid__000000000001eafa+(uuid__000000000001eafa<<(15)));({});});
});
uuid__000000000001eafa;
});}
#line 55 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_length_CL_ArrowU64String(char* uuid__000000000001eafb){unsigned long uuid__000000000001eafc;
return ({({({uuid__000000000001eafc=(0);({});})
;
({while(((((char*)(uuid__000000000001eafb))[uuid__000000000001eafc])!=(0))){((void)(({uuid__000000000001eafc=(uuid__000000000001eafc+(1));({});})));};});
});
uuid__000000000001eafc;
});}
#line 63 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsStringString(char* uuid__000000000001eafd,char* uuid__000000000001eafe){unsigned long uuid__000000000001eaff;
char uuid__000000000001eb00;
char uuid__000000000001eb01;
return ({({({uuid__000000000001eaff=(1);({});})
;
({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eafe))){((void)(((head_SB_string_CL_ArrowU8String(uuid__000000000001eafd))?({({({uuid__000000000001eb00=(head_SB_string_CL_ArrowU8String(uuid__000000000001eafd));({});})
;
({uuid__000000000001eb01=(head_SB_string_CL_ArrowU8String(uuid__000000000001eafe));({});})
;
});
((uuid__000000000001eb00==uuid__000000000001eb01)?({({uuid__000000000001eafd=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eafd));({});});
({uuid__000000000001eafe=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eafe));({});});
}):({({uuid__000000000001eaff=(0);({});});
({uuid__000000000001eafe="";({});});
}));
}):({({uuid__000000000001eaff=(0);({});});
({uuid__000000000001eafe="";({});});
}))));};});
});
uuid__000000000001eaff;
});}
#line 84 "PLATFORM/C/LIB/string.lm"
char* _DT_remove_SB_prefix_CL_ArrowStringConsStringString(char* uuid__000000000001eb02,char* uuid__000000000001eb03){return ({((_DT_has_SB_prefix_CL_ArrowU64ConsStringString(uuid__000000000001eb02,uuid__000000000001eb03))?({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eb03))){((void)(({({uuid__000000000001eb02=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb02));({});});
({uuid__000000000001eb03=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb03));({});});
})));};}):({}));
uuid__000000000001eb02;
});}
#line 94 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_has_SB_suffix_CL_ArrowU64ConsStringString(char* uuid__000000000001eb04,char* uuid__000000000001eb05){unsigned long uuid__000000000001eb06;
return ({({({uuid__000000000001eb06=(0);({});})
;
({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eb04))){((void)(({((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eb04,uuid__000000000001eb05))?({uuid__000000000001eb06=(1);({});}):({}));
({uuid__000000000001eb04=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb04));({});});
})));};});
});
uuid__000000000001eb06;
});}
#line 105 "PLATFORM/C/LIB/string.lm"
char* _DT_remove_SB_suffix_CL_ArrowStringConsStringString(char* uuid__000000000001eb07,char* uuid__000000000001eb08){LM_S uuid__000000000001eb09;
return ({({({uuid__000000000001eb09=({LM_S rvalue={3};rvalue;});({});})
;
({while((head_SB_string_CL_ArrowU8String(uuid__000000000001eb07))){((void)(({((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eb07,uuid__000000000001eb08))?({uuid__000000000001eb07="0";({});}):({uuid__000000000001eb09=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8((head_SB_string_CL_ArrowU8String(uuid__000000000001eb07))));rvalue;})));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(uuid__000000000001eb09));rvalue;});({});}));
({uuid__000000000001eb07=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb07));({});});
})));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001eb09));
});}
#line 121 "PLATFORM/C/LIB/string.lm"
char* _DT_replace_CL_ArrowStringConsStringConsStringString(char* uuid__000000000001eb0a,char* uuid__000000000001eb0b,char* uuid__000000000001eb0c){LM_S uuid__000000000001eb0d;
return ({({({uuid__000000000001eb0d=({LM_S rvalue={3};rvalue;});({});})
;
({while(((head_SB_string_CL_ArrowU8String(uuid__000000000001eb0a))!=(0))){((void)(((_DT_has_SB_prefix_CL_ArrowU64ConsStringString(uuid__000000000001eb0a,uuid__000000000001eb0b))?({({uuid__000000000001eb0a=(_DT_remove_SB_prefix_CL_ArrowStringConsStringString(uuid__000000000001eb0a,uuid__000000000001eb0b));({});});
({uuid__000000000001eb0d=(_AD__CL_ArrowSConsSS(uuid__000000000001eb0d,({LM_S rvalue={.field_0=2};rvalue.field_2001=uuid__000000000001eb0c;rvalue;})));({});});
}):({({uuid__000000000001eb0d=(_AD__CL_ArrowSConsSS(uuid__000000000001eb0d,({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8((head_SB_string_CL_ArrowU8String(uuid__000000000001eb0a))));rvalue;})));({});});
({uuid__000000000001eb0a=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb0a));({});});
}))));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001eb0d));
});}
#line 135 "PLATFORM/C/LIB/string.lm"
unsigned long _DT_contains_CL_ArrowU64ConsStringString(char* uuid__000000000001eb0e,char* uuid__000000000001eb0f){unsigned long uuid__000000000001eb10;
return ({({({uuid__000000000001eb10=(0);({});})
;
({while(((head_SB_string_CL_ArrowU8String(uuid__000000000001eb0e))!=(0))){((void)(({((_DT_has_SB_prefix_CL_ArrowU64ConsStringString(uuid__000000000001eb0e,uuid__000000000001eb0f))?({uuid__000000000001eb10=(1);({});}):({}));
({uuid__000000000001eb0e=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb0e));({});});
})));};});
});
uuid__000000000001eb10;
});}
#line 144 "PLATFORM/C/LIB/string.lm"
char* _AD__CL_ArrowStringConsStringString(char* uuid__000000000001eb11,char* uuid__000000000001eb12){return (clone_SB_rope_CL_ArrowStringS((_AD__CL_ArrowSConsSS(({LM_S rvalue={.field_0=2};rvalue.field_2001=uuid__000000000001eb11;rvalue;}),({LM_S rvalue={.field_0=2};rvalue.field_2001=uuid__000000000001eb12;rvalue;})))));}
#line 148 "PLATFORM/C/LIB/string.lm"
void print_CL_ArrowNilString(char* uuid__000000000001eb13){({while(((head_SB_string_CL_ArrowU8String(uuid__000000000001eb13))!=(0))){((void)(({(putchar(((unsigned int)((head_SB_string_CL_ArrowU8String(uuid__000000000001eb13))))));
({uuid__000000000001eb13=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb13));({});});
})));};});}
#line 155 "PLATFORM/C/LIB/string.lm"
unsigned long non_SB_zero_CL_ArrowU64String(char* uuid__000000000001eb14){return ((head_SB_string_CL_ArrowU8String(uuid__000000000001eb14))!=(0));}
#line 2 "PLATFORM/C/LIB/string.lsts"
LM_Ord cmp_CL_ArrowOrdConsStringString(char* uuid__000000000001eb15,char* uuid__000000000001eb16){signed int uuid__000000000001eb17;
return ({({uuid__000000000001eb17=(strcmp(((char*)(uuid__000000000001eb15)),((char*)(uuid__000000000001eb16))));({});})
;
((uuid__000000000001eb17<(0))?({LM_Ord rvalue={0};rvalue;}):((uuid__000000000001eb17>(0))?({LM_Ord rvalue={2};rvalue;}):({LM_Ord rvalue={1};rvalue;})));
});}
#line 9 "PLATFORM/C/LIB/string.lsts"
void print_CL_ArrowNilConsStringIO_CL__CL_File(FILE* uuid__000000000001eb18,char* uuid__000000000001eb19){({while(((head_SB_string_CL_ArrowU8String(uuid__000000000001eb19))!=(0))){((void)(({(fwrite(((char*)(uuid__000000000001eb19)),(1),(1),stdout));
({uuid__000000000001eb19=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb19));({});});
})));};});}
#line 16 "PLATFORM/C/LIB/string.lsts"
unsigned long _DT_is_SB_digit_CL_ArrowU64String(char* uuid__000000000001eb1a){unsigned long uuid__000000000001eb1b;
return ((non_SB_zero_CL_ArrowU64String(uuid__000000000001eb1a))?({({({uuid__000000000001eb1b=true_CL_U64;({});})
;
({while((((head_SB_string_CL_ArrowU8String(uuid__000000000001eb1a))!=(0))&&uuid__000000000001eb1b)){((void)(({({uuid__000000000001eb1b=(((48)<=(head_SB_string_CL_ArrowU8String(uuid__000000000001eb1a)))&&((head_SB_string_CL_ArrowU8String(uuid__000000000001eb1a))<=(57)));({});});
({uuid__000000000001eb1a=(tail_SB_string_CL_ArrowStringString(uuid__000000000001eb1a));({});});
})));};});
});
uuid__000000000001eb1b;
}):false_CL_U64);}
#line 4 "PLATFORM/C/LIB/smart-string.lm"
unsigned long non_SB_zero_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb1c){return ((uuid__000000000001eb1c.field_3)<(uuid__000000000001eb1c.field_2));}
#line 8 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString intern_CL_ArrowSmartStringString(char* uuid__000000000001eb1d){char* uuid__000000000001eb1e;
return ({({uuid__000000000001eb1e=(((char*)(uuid__000000000001eb1d))+(_DT_length_CL_ArrowU64String(uuid__000000000001eb1d)));({});})
;
({LM_SmartString rvalue={.field_0=0};rvalue.field_1=uuid__000000000001eb1e;rvalue.field_2=uuid__000000000001eb1e;rvalue.field_3=((char*)(uuid__000000000001eb1d));rvalue.field_4=((char*)(uuid__000000000001eb1d));rvalue;});
});}
#line 13 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString intern_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb1f){return uuid__000000000001eb1f;}
#line 17 "PLATFORM/C/LIB/smart-string.lm"
char* untern_CL_ArrowStringSmartString(LM_SmartString uuid__000000000001eb20){LM_S uuid__000000000001eb21;
char* uuid__000000000001eb22;
return ({({({({uuid__000000000001eb21=({LM_S rvalue={3};rvalue;});({});})
;
({uuid__000000000001eb22=(uuid__000000000001eb20.field_3);({});})
;
});
({while((uuid__000000000001eb22<(uuid__000000000001eb20.field_2))){((void)(({({uuid__000000000001eb21=(_AD__CL_ArrowSConsSS(uuid__000000000001eb21,({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8((uuid__000000000001eb22[(0)])));rvalue;})));({});});
({uuid__000000000001eb22=(uuid__000000000001eb22+(1));({});});
})));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001eb21));
});}
#line 29 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_length_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb23){return ((uuid__000000000001eb23.field_2)-(uuid__000000000001eb23.field_3));}
#line 33 "PLATFORM/C/LIB/smart-string.lm"
void print_CL_ArrowNilSmartString(LM_SmartString uuid__000000000001eb24){char* uuid__000000000001eb25;
({({uuid__000000000001eb25=(uuid__000000000001eb24.field_3);({});})
;
({while((uuid__000000000001eb25!=(uuid__000000000001eb24.field_2))){((void)(({(putchar(((unsigned int)((uuid__000000000001eb25[(0)])))));
({uuid__000000000001eb25=(uuid__000000000001eb25+(1));({});});
})));};});
});}
#line 41 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsU64ConsU64SmartString(LM_SmartString uuid__000000000001eb26,unsigned long uuid__000000000001eb27,unsigned long uuid__000000000001eb28){return (_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb26,((signed long)(uuid__000000000001eb27)),((signed long)(uuid__000000000001eb28))));}
#line 44 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsU64ConsI64SmartString(LM_SmartString uuid__000000000001eb29,signed long uuid__000000000001eb2a,unsigned long uuid__000000000001eb2b){return (_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb29,uuid__000000000001eb2a,((signed long)(uuid__000000000001eb2b))));}
#line 47 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsI64ConsU64SmartString(LM_SmartString uuid__000000000001eb2c,unsigned long uuid__000000000001eb2d,signed long uuid__000000000001eb2e){return (_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb2c,((signed long)(uuid__000000000001eb2d)),uuid__000000000001eb2e));}
#line 51 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(LM_SmartString uuid__000000000001eb2f,signed long uuid__000000000001eb30,signed long uuid__000000000001eb31){char* uuid__000000000001eb32;
char* uuid__000000000001eb33;
return ({({({({({({({((uuid__000000000001eb30<(0))?({uuid__000000000001eb30=(((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))))+uuid__000000000001eb30);({});}):({}));
((uuid__000000000001eb31==minimum_SB_I64_CL_I64)?({uuid__000000000001eb31=((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))));({});}):((uuid__000000000001eb31<(0))?({uuid__000000000001eb31=(((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))))+uuid__000000000001eb31);({});}):({})));
});
({uuid__000000000001eb32=((uuid__000000000001eb2f.field_3)+uuid__000000000001eb30);({});})
;
});
({uuid__000000000001eb33=((uuid__000000000001eb2f.field_3)+uuid__000000000001eb31);({});})
;
});
((uuid__000000000001eb33<uuid__000000000001eb32)?({({({({({({({(print_CL_ArrowNilString("["));
(print_CL_ArrowNilI64(uuid__000000000001eb30));
});
(print_CL_ArrowNilString(":"));
});
(print_CL_ArrowNilI64(uuid__000000000001eb31));
});
(print_CL_ArrowNilString("] of "));
});
(print_CL_ArrowNilU64((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))));
});
(print_CL_ArrowNilString("\n"));
});
(fail_CL_ArrowNeverString("Index Out Of Bounds: SmartString.[:]"));
}):({}));
});
((uuid__000000000001eb32<(uuid__000000000001eb2f.field_3))?({({({({({({({(print_CL_ArrowNilString("["));
(print_CL_ArrowNilI64(uuid__000000000001eb30));
});
(print_CL_ArrowNilString(":"));
});
(print_CL_ArrowNilI64(uuid__000000000001eb31));
});
(print_CL_ArrowNilString("] of "));
});
(print_CL_ArrowNilU64((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))));
});
(print_CL_ArrowNilString("\n"));
});
(fail_CL_ArrowNeverString("Index Out Of Bounds: SmartString.[:]"));
}):({}));
});
((uuid__000000000001eb33>(uuid__000000000001eb2f.field_2))?({({({({({({({(print_CL_ArrowNilString("["));
(print_CL_ArrowNilI64(uuid__000000000001eb30));
});
(print_CL_ArrowNilString(":"));
});
(print_CL_ArrowNilI64(uuid__000000000001eb31));
});
(print_CL_ArrowNilString("] of "));
});
(print_CL_ArrowNilU64((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb2f))));
});
(print_CL_ArrowNilString("\n"));
});
(fail_CL_ArrowNeverString("Index Out Of Bounds: SmartString.[:]"));
}):({}));
});
({LM_SmartString rvalue={.field_0=0};rvalue.field_1=(uuid__000000000001eb2f.field_1);rvalue.field_2=uuid__000000000001eb33;rvalue.field_3=uuid__000000000001eb32;rvalue.field_4=(uuid__000000000001eb2f.field_4);rvalue;});
});}
#line 79 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString tail_SB_string_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb34){return (_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb34,(1),((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb34))))));}
#line 83 "PLATFORM/C/LIB/smart-string.lm"
char _LB__RB__CL_ArrowU8ConsI64SmartString(LM_SmartString uuid__000000000001eb35,signed long uuid__000000000001eb36){return ({((uuid__000000000001eb36<(0))?({uuid__000000000001eb36=(((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb35))))+uuid__000000000001eb36);({});}):({}));
(_LB__RB__CL_ArrowU8ConsU64SmartString(uuid__000000000001eb35,((unsigned long)(uuid__000000000001eb36))));
});}
#line 90 "PLATFORM/C/LIB/smart-string.lm"
char _LB__RB__CL_ArrowU8ConsU64SmartString(LM_SmartString uuid__000000000001eb37,unsigned long uuid__000000000001eb38){char* uuid__000000000001eb39;
return ({({({({uuid__000000000001eb39=((uuid__000000000001eb37.field_3)+uuid__000000000001eb38);({});})
;
((uuid__000000000001eb39<(uuid__000000000001eb37.field_3))?(fail_CL_ArrowNeverString("Index Out Of Bounds: SmartString.[]")):({}));
});
((uuid__000000000001eb39>=(uuid__000000000001eb37.field_2))?(fail_CL_ArrowNeverString("Index Out Of Bounds: SmartString.[]")):({}));
});
(uuid__000000000001eb39[(0)]);
});}
#line 101 "PLATFORM/C/LIB/smart-string.lm"
char head_SB_string_CL_ArrowU8SmartString(LM_SmartString uuid__000000000001eb3a){return (_LB__RB__CL_ArrowU8ConsI64SmartString(uuid__000000000001eb3a,(0)));}
#line 105 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_has_SB_suffix_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3b,LM_SmartString uuid__000000000001eb3c){return (((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3b))>=(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3c)))&&(_EQ__EQ__CL_ArrowU64ConsSmartStringSmartString((_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb3b,((signed long)(((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3b))-(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3c))))),((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3b)))))),uuid__000000000001eb3c)));}
#line 113 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _DT_remove_SB_suffix_CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3d,LM_SmartString uuid__000000000001eb3e){return ((_DT_has_SB_suffix_CL_ArrowU64ConsSmartStringSmartString(uuid__000000000001eb3d,uuid__000000000001eb3e))?(_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb3d,(0),((signed long)(((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3d))-(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3e))))))):uuid__000000000001eb3d);}
#line 119 "PLATFORM/C/LIB/smart-string.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001eb3f,LM_SmartString uuid__000000000001eb40){return (((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb3f))>=(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb40)))&&(_EQ__EQ__CL_ArrowU64ConsSmartStringSmartString((_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb3f,(0),((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb40)))))),uuid__000000000001eb40)));}
#line 127 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _DT_remove_SB_prefix_CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb41,LM_SmartString uuid__000000000001eb42){return ((_DT_has_SB_prefix_CL_ArrowU64ConsSmartStringSmartString(uuid__000000000001eb41,uuid__000000000001eb42))?(_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001eb41,((signed long)((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb42)))),minimum_SB_I64_CL_I64)):uuid__000000000001eb41);}
#line 133 "PLATFORM/C/LIB/smart-string.lm"
char* _DT_replace_CL_ArrowStringConsSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb43,LM_SmartString uuid__000000000001eb44,LM_SmartString uuid__000000000001eb45){LM_S uuid__000000000001eb46;
return ({({({uuid__000000000001eb46=({LM_S rvalue={3};rvalue;});({});})
;
({while(((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb43))>(0))){((void)(((_DT_has_SB_prefix_CL_ArrowU64ConsSmartStringSmartString(uuid__000000000001eb43,uuid__000000000001eb44))?({({uuid__000000000001eb43=(_DT_remove_SB_prefix_CL_ArrowSmartStringConsSmartStringSmartString(uuid__000000000001eb43,uuid__000000000001eb44));({});});
({uuid__000000000001eb46=(_AD__CL_ArrowSConsSS(uuid__000000000001eb46,({LM_S rvalue={.field_0=2};rvalue.field_2001=(untern_CL_ArrowStringSmartString(uuid__000000000001eb45));rvalue;})));({});});
}):({({uuid__000000000001eb46=(_AD__CL_ArrowSConsSS(uuid__000000000001eb46,({LM_S rvalue={.field_0=2};rvalue.field_2001=(clone_SB_rope_CL_ArrowStringU8((head_SB_string_CL_ArrowU8SmartString(uuid__000000000001eb43))));rvalue;})));({});});
({uuid__000000000001eb43=(tail_SB_string_CL_ArrowSmartStringSmartString(uuid__000000000001eb43));({});});
}))));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001eb46));
});}
#line 148 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString _AD__CL_ArrowSmartStringConsSmartStringSmartString(LM_SmartString uuid__000000000001eb47,LM_SmartString uuid__000000000001eb48){unsigned long uuid__000000000001eb49;
char* uuid__000000000001eb4a;
unsigned long uuid__000000000001eb4b;
unsigned long uuid__000000000001eb4c;
return ({({({({({({({({uuid__000000000001eb49=((_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb47))+(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb48)));({});})
;
({uuid__000000000001eb4a=((char*)((malloc((uuid__000000000001eb49+(1))))));({});})
;
});
({uuid__000000000001eb4b=(0);({});})
;
});
({while((uuid__000000000001eb4b<(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb47)))){((void)(({(uuid__000000000001eb4a[uuid__000000000001eb4b]=(_LB__RB__CL_ArrowU8ConsU64SmartString(uuid__000000000001eb47,uuid__000000000001eb4b)));
({uuid__000000000001eb4b=(uuid__000000000001eb4b+(1));({});});
})));};});
});
({uuid__000000000001eb4c=(0);({});})
;
});
({while((uuid__000000000001eb4c<(_DT_length_CL_ArrowU64SmartString(uuid__000000000001eb48)))){((void)(({(uuid__000000000001eb4a[(uuid__000000000001eb4b+uuid__000000000001eb4c)]=(_LB__RB__CL_ArrowU8ConsU64SmartString(uuid__000000000001eb48,uuid__000000000001eb4c)));
({uuid__000000000001eb4c=(uuid__000000000001eb4c+(1));({});});
})));};});
});
(uuid__000000000001eb4a[uuid__000000000001eb49]=(0));
});
({LM_SmartString rvalue={.field_0=0};rvalue.field_1=(uuid__000000000001eb4a+uuid__000000000001eb49);rvalue.field_2=(uuid__000000000001eb4a+uuid__000000000001eb49);rvalue.field_3=uuid__000000000001eb4a;rvalue.field_4=uuid__000000000001eb4a;rvalue;});
});}
#line 165 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringSmartString(LM_SmartString uuid__000000000001eb4d){return uuid__000000000001eb4d;}
#line 166 "PLATFORM/C/LIB/smart-string.lm"
LM_SmartString to_SB_smart_SB_string_CL_ArrowSmartStringString(char* uuid__000000000001eb4e){return (intern_CL_ArrowSmartStringString(uuid__000000000001eb4e));}
#line 168 "PLATFORM/C/LIB/smart-string.lm"
unsigned long deep_SB_hash_CL_ArrowU64SmartString(LM_SmartString uuid__000000000001eb4f){unsigned long uuid__000000000001eb50;
unsigned long uuid__000000000001eb51;
char* uuid__000000000001eb52;
return ({({({({({({({({uuid__000000000001eb50=(0);({});})
;
({uuid__000000000001eb51=(0);({});})
;
});
({uuid__000000000001eb52=(uuid__000000000001eb4f.field_3);({});})
;
});
({while((uuid__000000000001eb52<(uuid__000000000001eb4f.field_2))){((void)(({({({({uuid__000000000001eb51=(uuid__000000000001eb51+((unsigned long)((uuid__000000000001eb52[(0)]))));({});});
({uuid__000000000001eb51=(uuid__000000000001eb51+(uuid__000000000001eb51<<(10)));({});});
});
({uuid__000000000001eb51=(uuid__000000000001eb51^(uuid__000000000001eb51>>(6)));({});});
});
({uuid__000000000001eb52=(uuid__000000000001eb52+(1));({});});
})));};});
});
({uuid__000000000001eb51=(uuid__000000000001eb51+(uuid__000000000001eb51<<(3)));({});});
});
({uuid__000000000001eb51=(uuid__000000000001eb51^(uuid__000000000001eb51>>(11)));({});});
});
({uuid__000000000001eb51=(uuid__000000000001eb51+(uuid__000000000001eb51<<(15)));({});});
});
uuid__000000000001eb51;
});}
#line 2 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsSmartStringString(char* uuid__000000000001eb53,LM_SmartString uuid__000000000001eb54){return (cmp_CL_ArrowOrdConsSmartStringSmartString((intern_CL_ArrowSmartStringString(uuid__000000000001eb53)),uuid__000000000001eb54));}
#line 5 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb55,LM_SmartString uuid__000000000001eb56){return (_EQ__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb55,uuid__000000000001eb56)),({LM_Ord rvalue={1};rvalue;})));}
#line 6 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb57,LM_SmartString uuid__000000000001eb58){return (_EX__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb57,uuid__000000000001eb58)),({LM_Ord rvalue={1};rvalue;})));}
#line 7 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb59,LM_SmartString uuid__000000000001eb5a){return (_LT__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb59,uuid__000000000001eb5a)),({LM_Ord rvalue={1};rvalue;})));}
#line 8 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5b,LM_SmartString uuid__000000000001eb5c){return (_LT__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb5b,uuid__000000000001eb5c)),({LM_Ord rvalue={1};rvalue;})));}
#line 9 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5d,LM_SmartString uuid__000000000001eb5e){return (_GT__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb5d,uuid__000000000001eb5e)),({LM_Ord rvalue={1};rvalue;})));}
#line 10 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsSmartStringString(char* uuid__000000000001eb5f,LM_SmartString uuid__000000000001eb60){return (_GT__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringString(uuid__000000000001eb5f,uuid__000000000001eb60)),({LM_Ord rvalue={1};rvalue;})));}
#line 12 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsStringSmartString(LM_SmartString uuid__000000000001eb61,char* uuid__000000000001eb62){return (cmp_CL_ArrowOrdConsSmartStringSmartString(uuid__000000000001eb61,(intern_CL_ArrowSmartStringString(uuid__000000000001eb62))));}
#line 15 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb63,char* uuid__000000000001eb64){return (_EQ__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb63,uuid__000000000001eb64)),({LM_Ord rvalue={1};rvalue;})));}
#line 16 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb65,char* uuid__000000000001eb66){return (_EX__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb65,uuid__000000000001eb66)),({LM_Ord rvalue={1};rvalue;})));}
#line 17 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb67,char* uuid__000000000001eb68){return (_LT__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb67,uuid__000000000001eb68)),({LM_Ord rvalue={1};rvalue;})));}
#line 18 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb69,char* uuid__000000000001eb6a){return (_LT__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb69,uuid__000000000001eb6a)),({LM_Ord rvalue={1};rvalue;})));}
#line 19 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb6b,char* uuid__000000000001eb6c){return (_GT__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb6b,uuid__000000000001eb6c)),({LM_Ord rvalue={1};rvalue;})));}
#line 20 "PLATFORM/C/LIB/smart-string.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsStringSmartString(LM_SmartString uuid__000000000001eb6d,char* uuid__000000000001eb6e){return (_GT__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringSmartString(uuid__000000000001eb6d,uuid__000000000001eb6e)),({LM_Ord rvalue={1};rvalue;})));}
#line 22 "PLATFORM/C/LIB/smart-string.lsts"
LM_Ord cmp_CL_ArrowOrdConsSmartStringSmartString(LM_SmartString uuid__000000000001eb6f,LM_SmartString uuid__000000000001eb70){LM_Ord uuid__000000000001eb71;
char* uuid__000000000001eb72;
char* uuid__000000000001eb73;
return ({({({uuid__000000000001eb71=({LM_Ord rvalue={1};rvalue;});({});})
;
((!(is_CL_ArrowU64ConsSmartStringSmartString(uuid__000000000001eb6f,uuid__000000000001eb70)))?({({({({({uuid__000000000001eb72=(uuid__000000000001eb6f.field_3);({});})
;
({uuid__000000000001eb73=(uuid__000000000001eb70.field_3);({});})
;
});
({while(((uuid__000000000001eb72<(uuid__000000000001eb6f.field_2))&&(uuid__000000000001eb73<(uuid__000000000001eb70.field_2)))){((void)((((uuid__000000000001eb72[(0)])<(uuid__000000000001eb73[(0)]))?({({({uuid__000000000001eb71=({LM_Ord rvalue={0};rvalue;});({});});
({uuid__000000000001eb72=(uuid__000000000001eb6f.field_2);({});});
});
({uuid__000000000001eb73=(uuid__000000000001eb70.field_2);({});});
}):(((uuid__000000000001eb72[(0)])>(uuid__000000000001eb73[(0)]))?({({({uuid__000000000001eb71=({LM_Ord rvalue={2};rvalue;});({});});
({uuid__000000000001eb72=(uuid__000000000001eb6f.field_2);({});});
});
({uuid__000000000001eb73=(uuid__000000000001eb70.field_2);({});});
}):({({uuid__000000000001eb72=(uuid__000000000001eb72+(1));({});});
({uuid__000000000001eb73=(uuid__000000000001eb73+(1));({});});
})))));};});
});
((uuid__000000000001eb72<(uuid__000000000001eb6f.field_2))?({uuid__000000000001eb71=({LM_Ord rvalue={2};rvalue;});({});}):({}));
});
((uuid__000000000001eb73<(uuid__000000000001eb70.field_2))?({uuid__000000000001eb71=({LM_Ord rvalue={0};rvalue;});({});}):({}));
}):({}));
});
uuid__000000000001eb71;
});}
#line 47 "PLATFORM/C/LIB/smart-string.lsts"
void print_CL_ArrowNilConsSmartStringIO_CL__CL_File(FILE* uuid__000000000001eb74,LM_SmartString uuid__000000000001eb75){char* uuid__000000000001eb76;
({({uuid__000000000001eb76=(uuid__000000000001eb75.field_3);({});})
;
({while((uuid__000000000001eb76!=(uuid__000000000001eb75.field_2))){((void)(({(fwrite(uuid__000000000001eb76,(1),(1),stdout));
({uuid__000000000001eb76=(uuid__000000000001eb76+(1));({});});
})));};});
});}
#line 3 "PLATFORM/C/LIB/s.lsts"
unsigned long non_SB_zero_CL_ArrowU64S(LM_S uuid__000000000001eb77){return ((uuid__000000000001eb77.field_0)!=(({LM_S rvalue={3};rvalue;}).field_0));}
#line 5 "PLATFORM/C/LIB/s.lsts"
void print_CL_ArrowNilConsSIO_CL__CL_File(FILE* uuid__000000000001eb78,LM_S uuid__000000000001eb79){LM_S uuid__000000000001eb7a;
LM_S uuid__000000000001eb7b;
LM_S uuid__000000000001eb7e;
char uuid__000000000001eb7f;
char* uuid__000000000001eb80;
LM_S uuid__000000000001eb81;
char uuid__000000000001eb82;
LM_S uuid__000000000001eb83;
LM_S uuid__000000000001eb84;
LM_S uuid__000000000001eb8b;
char uuid__000000000001eb8c;
char uuid__000000000001eb8d;
({({uuid__000000000001eb7a=uuid__000000000001eb79;({});})
;
(({({uuid__000000000001eb7b=uuid__000000000001eb7a;({});})
;
((uuid__000000000001eb7b.field_0)==(3));
})?(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb7c)):(({({({({uuid__000000000001eb7e=uuid__000000000001eb7a;({});})
;
({uuid__000000000001eb7f=(0);({});})
;
});
(((uuid__000000000001eb7e.field_0)==(2))?(({({uuid__000000000001eb80=(((LM_S)(uuid__000000000001eb7e)).field_2001);({});})
;
1;
})?({uuid__000000000001eb7f=(1);({});}):({})):({}));
});
(uuid__000000000001eb7f==(1));
})?(print_CL_ArrowNilConsStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb80)):(({({({({uuid__000000000001eb81=uuid__000000000001eb7a;({});})
;
({uuid__000000000001eb82=(0);({});})
;
});
(((uuid__000000000001eb81.field_0)==(1))?(({({uuid__000000000001eb83=(*(((LM_S)(uuid__000000000001eb81)).field_1001));({});})
;
1;
})?(({({uuid__000000000001eb84=(*(((LM_S)(uuid__000000000001eb81)).field_1002));({});})
;
1;
})?({uuid__000000000001eb82=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eb82==(1));
})?({({({({(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb85));
(print_CL_ArrowNilConsSIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb84));
});
(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb87));
});
(print_CL_ArrowNilConsSIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb83));
});
(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb89));
}):(({({({({uuid__000000000001eb8b=uuid__000000000001eb7a;({});})
;
({uuid__000000000001eb8c=(0);({});})
;
});
(((uuid__000000000001eb8b.field_0)==(0))?(({({uuid__000000000001eb8d=(*(((LM_S)(uuid__000000000001eb8b)).field_1));({});})
;
1;
})?({uuid__000000000001eb8c=(1);({});}):({})):({}));
});
(uuid__000000000001eb8c==(1));
})?({({(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb8e));
(print_CL_ArrowNilConsU64IO_CL__CL_File(uuid__000000000001eb78,((unsigned long)(uuid__000000000001eb8d))));
});
(print_CL_ArrowNilConsSmartStringIO_CL__CL_File(uuid__000000000001eb78,uuid__000000000001eb90));
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/s.lsts Line: 6 Column: 4"))))));
});}
#line 18 "PLATFORM/C/LIB/s.lsts"
LM_S _AD__CL_ArrowSConsSS(LM_S uuid__000000000001eb92,LM_S uuid__000000000001eb93){return ({((non_SB_zero_CL_ArrowU64S(uuid__000000000001eb93))?((non_SB_zero_CL_ArrowU64S(uuid__000000000001eb92))?({uuid__000000000001eb92=({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(uuid__000000000001eb93));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(uuid__000000000001eb92));rvalue;});({});}):({uuid__000000000001eb92=uuid__000000000001eb93;({});})):({}));
uuid__000000000001eb92;
});}
#line 26 "PLATFORM/C/LIB/s.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSS(LM_S uuid__000000000001eb94,LM_S uuid__000000000001eb95){LM_TupleSS uuid__000000000001eb96;
LM_TupleSS uuid__000000000001eba9;
char uuid__000000000001ebaa;
LM_S uuid__000000000001ebab;
LM_S uuid__000000000001ebac;
LM_TupleSS uuid__000000000001ebad;
char uuid__000000000001ebae;
LM_S uuid__000000000001ebaf;
char uuid__000000000001ebb0;
char* uuid__000000000001ebb1;
LM_S uuid__000000000001ebb2;
char uuid__000000000001ebb3;
char* uuid__000000000001ebb4;
LM_TupleSS uuid__000000000001ebb5;
char uuid__000000000001ebb6;
LM_S uuid__000000000001ebb7;
char uuid__000000000001ebb8;
LM_S uuid__000000000001ebb9;
LM_S uuid__000000000001ebba;
LM_S uuid__000000000001ebbb;
char uuid__000000000001ebbc;
LM_S uuid__000000000001ebbd;
LM_S uuid__000000000001ebbe;
LM_TupleSS uuid__000000000001ebbf;
char uuid__000000000001ebc0;
LM_S uuid__000000000001ebc1;
char uuid__000000000001ebc2;
char uuid__000000000001ebc3;
LM_S uuid__000000000001ebc4;
char uuid__000000000001ebc5;
char uuid__000000000001ebc6;
return ({({uuid__000000000001eb96=({LM_TupleSS rvalue={.field_0=0};rvalue.field_1=uuid__000000000001eb95;rvalue.field_2=uuid__000000000001eb94;rvalue;});({});})
;
(({({({({uuid__000000000001eba9=uuid__000000000001eb96;({});})
;
({uuid__000000000001ebaa=(0);({});})
;
});
(((uuid__000000000001eba9.field_0)==(0))?(({({uuid__000000000001ebab=(((LM_TupleSS)(uuid__000000000001eba9)).field_1);({});})
;
((uuid__000000000001ebab.field_0)==(3));
})?(({({uuid__000000000001ebac=(((LM_TupleSS)(uuid__000000000001eba9)).field_2);({});})
;
((uuid__000000000001ebac.field_0)==(3));
})?({uuid__000000000001ebaa=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebaa==(1));
})?true_CL_U64:(({({({({uuid__000000000001ebad=uuid__000000000001eb96;({});})
;
({uuid__000000000001ebae=(0);({});})
;
});
(((uuid__000000000001ebad.field_0)==(0))?(({({({({uuid__000000000001ebaf=(((LM_TupleSS)(uuid__000000000001ebad)).field_1);({});})
;
({uuid__000000000001ebb0=(0);({});})
;
});
(((uuid__000000000001ebaf.field_0)==(2))?(({({uuid__000000000001ebb1=(((LM_S)(uuid__000000000001ebaf)).field_2001);({});})
;
1;
})?({uuid__000000000001ebb0=(1);({});}):({})):({}));
});
(uuid__000000000001ebb0==(1));
})?(({({({({uuid__000000000001ebb2=(((LM_TupleSS)(uuid__000000000001ebad)).field_2);({});})
;
({uuid__000000000001ebb3=(0);({});})
;
});
(((uuid__000000000001ebb2.field_0)==(2))?(({({uuid__000000000001ebb4=(((LM_S)(uuid__000000000001ebb2)).field_2001);({});})
;
1;
})?({uuid__000000000001ebb3=(1);({});}):({})):({}));
});
(uuid__000000000001ebb3==(1));
})?({uuid__000000000001ebae=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebae==(1));
})?(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ebb4,uuid__000000000001ebb1)):(({({({({uuid__000000000001ebb5=uuid__000000000001eb96;({});})
;
({uuid__000000000001ebb6=(0);({});})
;
});
(((uuid__000000000001ebb5.field_0)==(0))?(({({({({uuid__000000000001ebb7=(((LM_TupleSS)(uuid__000000000001ebb5)).field_1);({});})
;
({uuid__000000000001ebb8=(0);({});})
;
});
(((uuid__000000000001ebb7.field_0)==(1))?(({({uuid__000000000001ebb9=(*(((LM_S)(uuid__000000000001ebb7)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ebba=(*(((LM_S)(uuid__000000000001ebb7)).field_1002));({});})
;
1;
})?({uuid__000000000001ebb8=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebb8==(1));
})?(({({({({uuid__000000000001ebbb=(((LM_TupleSS)(uuid__000000000001ebb5)).field_2);({});})
;
({uuid__000000000001ebbc=(0);({});})
;
});
(((uuid__000000000001ebbb.field_0)==(1))?(({({uuid__000000000001ebbd=(*(((LM_S)(uuid__000000000001ebbb)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ebbe=(*(((LM_S)(uuid__000000000001ebbb)).field_1002));({});})
;
1;
})?({uuid__000000000001ebbc=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebbc==(1));
})?({uuid__000000000001ebb6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebb6==(1));
})?((_EQ__EQ__CL_ArrowU64ConsSS(uuid__000000000001ebbe,uuid__000000000001ebba))&&(_EQ__EQ__CL_ArrowU64ConsSS(uuid__000000000001ebbd,uuid__000000000001ebb9))):(({({({({uuid__000000000001ebbf=uuid__000000000001eb96;({});})
;
({uuid__000000000001ebc0=(0);({});})
;
});
(((uuid__000000000001ebbf.field_0)==(0))?(({({({({uuid__000000000001ebc1=(((LM_TupleSS)(uuid__000000000001ebbf)).field_1);({});})
;
({uuid__000000000001ebc2=(0);({});})
;
});
(((uuid__000000000001ebc1.field_0)==(0))?(({({uuid__000000000001ebc3=(*(((LM_S)(uuid__000000000001ebc1)).field_1));({});})
;
1;
})?({uuid__000000000001ebc2=(1);({});}):({})):({}));
});
(uuid__000000000001ebc2==(1));
})?(({({({({uuid__000000000001ebc4=(((LM_TupleSS)(uuid__000000000001ebbf)).field_2);({});})
;
({uuid__000000000001ebc5=(0);({});})
;
});
(((uuid__000000000001ebc4.field_0)==(0))?(({({uuid__000000000001ebc6=(*(((LM_S)(uuid__000000000001ebc4)).field_1));({});})
;
1;
})?({uuid__000000000001ebc5=(1);({});}):({})):({}));
});
(uuid__000000000001ebc5==(1));
})?({uuid__000000000001ebc0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebc0==(1));
})?(((char)(uuid__000000000001ebc6))==((char)(uuid__000000000001ebc3))):(1?false_CL_U64:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/s.lsts Line: 27 Column: 4"));unsigned long rvalue;rvalue;}))))));
});}
#line 38 "PLATFORM/C/LIB/s.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsSS(LM_S uuid__000000000001ebc7,LM_S uuid__000000000001ebc8){return (!(_EQ__EQ__CL_ArrowU64ConsSS(uuid__000000000001ebc7,uuid__000000000001ebc8)));}
#line 134 "PLATFORM/C/LIB/list.lm"
char* _DT_join_CL_ArrowStringConsStringListString(LM_ListString uuid__000000000001ebc9,char* uuid__000000000001ebca){LM_S uuid__000000000001ebcb;
return ({({({uuid__000000000001ebcb=({LM_S rvalue={3};rvalue;});({});})
;
({while((non_SB_zero_CL_ArrowU64ListString(uuid__000000000001ebc9))){((void)(({({({uuid__000000000001ebcb=(_AD__CL_ArrowSConsSS(uuid__000000000001ebcb,({LM_S rvalue={.field_0=2};rvalue.field_2001=(head_CL_ArrowStringListString(uuid__000000000001ebc9));rvalue;})));({});});
({uuid__000000000001ebc9=(tail_CL_ArrowListStringListString(uuid__000000000001ebc9));({});});
});
((non_SB_zero_CL_ArrowU64ListString(uuid__000000000001ebc9))?({uuid__000000000001ebcb=(_AD__CL_ArrowSConsSS(uuid__000000000001ebcb,({LM_S rvalue={.field_0=2};rvalue.field_2001=uuid__000000000001ebca;rvalue;})));({});}):({}));
})));};});
});
(clone_SB_rope_CL_ArrowStringS(uuid__000000000001ebcb));
});}
#line 37 "PLATFORM/C/LIB/io.lm"
void exit_CL_ArrowNilU64(unsigned long uuid__000000000001ebcc){(exit(((unsigned int)(uuid__000000000001ebcc))));}
#line 39 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverConsStringString(char* uuid__000000000001ebcd,char* uuid__000000000001ebce){return ({({({(print_CL_ArrowNilString(uuid__000000000001ebcd));
(print_CL_ArrowNilString(" at "));
});
(print_CL_ArrowNilString(uuid__000000000001ebce));
});
(exit((1)));
});}
#line 44 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverSmartString(LM_SmartString uuid__000000000001ebcf){return ({(print_CL_ArrowNilSmartString(uuid__000000000001ebcf));
(exit((1)));
});}
#line 48 "PLATFORM/C/LIB/io.lm"
void fail_CL_ArrowNeverString(char* uuid__000000000001ebd0){return ({(print_CL_ArrowNilString(uuid__000000000001ebd0));
(exit((1)));
});}
#line 53 "PLATFORM/C/LIB/io.lm"
char* read_SB_file_CL_ArrowStringString(char* uuid__000000000001ebd1){unsigned long uuid__000000000001ebd2;
char* uuid__000000000001ebd3;
FILE* uuid__000000000001ebd4;
unsigned long uuid__000000000001ebd5;
return ({({({({({({({({({uuid__000000000001ebd2=(0);({});})
;
({uuid__000000000001ebd3=((char*)((malloc((1024)))));({});})
;
});
({uuid__000000000001ebd4=(fopen(((char*)(uuid__000000000001ebd1)),((char*)("r"))));({});})
;
});
((((unsigned long)(uuid__000000000001ebd4))==(0))?({({({(print_CL_ArrowNilString("Unable To Read From File: "));
(print_CL_ArrowNilString(uuid__000000000001ebd1));
});
(print_CL_ArrowNilString("\n"));
});
(exit_CL_ArrowNilU64((1)));
}):({}));
});
({uuid__000000000001ebd5=(1);({});})
;
});
({while(uuid__000000000001ebd5){((void)(({({({uuid__000000000001ebd5=(fread((uuid__000000000001ebd3+uuid__000000000001ebd2),(1),(1023),uuid__000000000001ebd4));({});});
({uuid__000000000001ebd2=(uuid__000000000001ebd2+uuid__000000000001ebd5);({});});
});
({uuid__000000000001ebd3=((char*)((realloc(uuid__000000000001ebd3,(uuid__000000000001ebd2+(1023))))));({});});
})));};});
});
(fclose(uuid__000000000001ebd4));
});
(uuid__000000000001ebd3[uuid__000000000001ebd2]=(0));
});
((char*)(uuid__000000000001ebd3));
});}
#line 71 "PLATFORM/C/LIB/io.lm"
void write_SB_file_CL_ArrowNilConsStringString(char* uuid__000000000001ebd6,char* uuid__000000000001ebd7){FILE* uuid__000000000001ebd8;
({({({({uuid__000000000001ebd8=(fopen(((char*)(uuid__000000000001ebd6)),((char*)("w"))));({});})
;
(fwrite(((char*)(uuid__000000000001ebd7)),(1),(_DT_length_CL_ArrowU64String(uuid__000000000001ebd7)),uuid__000000000001ebd8));
});
(fclose(uuid__000000000001ebd8));
});
({});
});}
#line 77 "PLATFORM/C/LIB/io.lm"
unsigned long file_SB_exists_CL_ArrowU64String(char* uuid__000000000001ebd9){FILE* uuid__000000000001ebda;
unsigned long uuid__000000000001ebdb;
return ({({({({uuid__000000000001ebda=(fopen(((char*)(uuid__000000000001ebd9)),((char*)("r"))));({});})
;
({uuid__000000000001ebdb=(0);({});})
;
});
((((unsigned long)(uuid__000000000001ebda))!=(0))?({({({uuid__000000000001ebdb=(1);({});});
(fclose(uuid__000000000001ebda));
});
({});
}):({}));
});
uuid__000000000001ebdb;
});}
#line 2 "PLATFORM/C/LIB/io.lsts"
char* _DT_file_SB_extension_CL_ArrowStringString(char* uuid__000000000001ebdc){return ({({while((((head_SB_string_CL_ArrowU8String(uuid__000000000001ebdc))!=(0))&&((head_SB_string_CL_ArrowU8String(uuid__000000000001ebdc))!=(46)))){((void)(({uuid__000000000001ebdc=(tail_SB_string_CL_ArrowStringString(uuid__000000000001ebdc));({});})));};});
uuid__000000000001ebdc;
});}
#line 15 "PLATFORM/C/LIB/regex.lm"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsRegexSmartString(LM_SmartString uuid__000000000001ebdd,regex_t uuid__000000000001ebde){int uuid__000000000001ebdf;
return ({({uuid__000000000001ebdf=(regexec(((regex_t * )((&uuid__000000000001ebde))),((char *)((uuid__000000000001ebdd.field_3))),((size_t )((0))),((regmatch_t *)((0))),((int)((0)))));({});})
;
(((unsigned long)(uuid__000000000001ebdf))==(0));
});}
#line 26 "PLATFORM/C/LIB/regex.lm"
LM_SmartString _DT_remove_SB_prefix_CL_ArrowSmartStringConsRegexSmartString(LM_SmartString uuid__000000000001ebe0,regex_t uuid__000000000001ebe1){regmatch_t  uuid__000000000001ebe2[1];
int uuid__000000000001ebe3;
return ({({({;
({uuid__000000000001ebe3=(regexec(((regex_t * )((&uuid__000000000001ebe1))),((char *)((uuid__000000000001ebe0.field_3))),((size_t )((1))),((regmatch_t *)(uuid__000000000001ebe2)),((int)((0)))));({});})
;
});
((((uuid__000000000001ebe2[(0)]).rm_so)!=(0))?(fail_CL_ArrowNeverString("Remove SmartString Prefix By Regex Not A Prefix")):({}));
});
(_LB__CL__RB__CL_ArrowSmartStringConsI64ConsI64SmartString(uuid__000000000001ebe0,(0),((signed long)(((uuid__000000000001ebe2[(0)]).rm_eo)))));
});}
#line 9 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe4,LM_Ord uuid__000000000001ebe5){return ((uuid__000000000001ebe4.field_0)==(uuid__000000000001ebe5.field_0));}
#line 10 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EX__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe6,LM_Ord uuid__000000000001ebe7){return ((uuid__000000000001ebe6.field_0)!=(uuid__000000000001ebe7.field_0));}
#line 11 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _LT__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebe8,LM_Ord uuid__000000000001ebe9){return ((uuid__000000000001ebe8.field_0)<(uuid__000000000001ebe9.field_0));}
#line 12 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _LT__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebea,LM_Ord uuid__000000000001ebeb){return ((uuid__000000000001ebea.field_0)<=(uuid__000000000001ebeb.field_0));}
#line 13 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _GT__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebec,LM_Ord uuid__000000000001ebed){return ((uuid__000000000001ebec.field_0)>(uuid__000000000001ebed.field_0));}
#line 14 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _GT__EQ__CL_ArrowU64ConsOrdOrd(LM_Ord uuid__000000000001ebee,LM_Ord uuid__000000000001ebef){return ((uuid__000000000001ebee.field_0)>=(uuid__000000000001ebef.field_0));}
#line 23 "PLATFORM/C/LIB/cmp.lsts"
LM_Ord _AM__AM__CL_ArrowOrdConsOrdOrd(LM_Ord uuid__000000000001ebf0,LM_Ord uuid__000000000001ebf1){return ((_EQ__EQ__CL_ArrowU64ConsOrdOrd(uuid__000000000001ebf0,({LM_Ord rvalue={1};rvalue;})))?uuid__000000000001ebf1:uuid__000000000001ebf0);}
#line 40 "PLATFORM/C/LIB/umbra.lsts"
unsigned long _DT_length_CL_ArrowU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebf2){return ((unsigned long)((uuid__000000000001ebf2->field_2)));}
#line 46 "PLATFORM/C/LIB/umbra.lsts"
char _LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebf3,unsigned long uuid__000000000001ebf4){LM_UmbraShortLong* uuid__000000000001ebf5;
LM_UmbraShortLong* uuid__000000000001ebf6;
char uuid__000000000001ebf7;
char* uuid__000000000001ebf8;
LM_UmbraShortLong* uuid__000000000001ebf9;
char uuid__000000000001ebfa;
char* uuid__000000000001ebfb;
char* uuid__000000000001ebfc;
return ({({uuid__000000000001ebf5=(&uuid__000000000001ebf3->field_1);({});})
;
(({({({({uuid__000000000001ebf6=uuid__000000000001ebf5;({});})
;
({uuid__000000000001ebf7=(0);({});})
;
});
(((uuid__000000000001ebf6->field_0)==(1))?(({({uuid__000000000001ebf8=(((LM_UmbraShortLong*)(uuid__000000000001ebf6))->field_1001);({});})
;
1;
})?({uuid__000000000001ebf7=(1);({});}):({})):({}));
});
(uuid__000000000001ebf7==(1));
})?(uuid__000000000001ebf8[uuid__000000000001ebf4]):(({({({({uuid__000000000001ebf9=uuid__000000000001ebf5;({});})
;
({uuid__000000000001ebfa=(0);({});})
;
});
(((uuid__000000000001ebf9->field_0)==(0))?(({({uuid__000000000001ebfb=(((LM_UmbraShortLong*)(uuid__000000000001ebf9))->field_1);({});})
;
1;
})?(({({uuid__000000000001ebfc=(((LM_UmbraShortLong*)(uuid__000000000001ebf9))->field_2);({});})
;
1;
})?({uuid__000000000001ebfa=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ebfa==(1));
})?((uuid__000000000001ebf4<(4))?(uuid__000000000001ebfc[uuid__000000000001ebf4]):(uuid__000000000001ebfb[uuid__000000000001ebf4])):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/umbra.lsts Line: 47 Column: 5"));char rvalue;rvalue;})));
});}
#line 60 "PLATFORM/C/LIB/umbra.lsts"
void set_LB__RB__CL_ArrowNilConsU8ConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ebfd,unsigned long uuid__000000000001ebfe,char uuid__000000000001ebff){LM_UmbraShortLong* uuid__000000000001ec00;
LM_UmbraShortLong* uuid__000000000001ec01;
char uuid__000000000001ec02;
char* uuid__000000000001ec03;
LM_UmbraShortLong* uuid__000000000001ec04;
char uuid__000000000001ec05;
char* uuid__000000000001ec06;
char* uuid__000000000001ec07;
({({uuid__000000000001ec00=(&uuid__000000000001ebfd->field_1);({});})
;
(({({({({uuid__000000000001ec01=uuid__000000000001ec00;({});})
;
({uuid__000000000001ec02=(0);({});})
;
});
(((uuid__000000000001ec01->field_0)==(1))?(({({uuid__000000000001ec03=(((LM_UmbraShortLong*)(uuid__000000000001ec01))->field_1001);({});})
;
1;
})?({uuid__000000000001ec02=(1);({});}):({})):({}));
});
(uuid__000000000001ec02==(1));
})?(uuid__000000000001ec03[uuid__000000000001ebfe]=uuid__000000000001ebff):(({({({({uuid__000000000001ec04=uuid__000000000001ec00;({});})
;
({uuid__000000000001ec05=(0);({});})
;
});
(((uuid__000000000001ec04->field_0)==(0))?(({({uuid__000000000001ec06=(((LM_UmbraShortLong*)(uuid__000000000001ec04))->field_1);({});})
;
1;
})?(({({uuid__000000000001ec07=(((LM_UmbraShortLong*)(uuid__000000000001ec04))->field_2);({});})
;
1;
})?({uuid__000000000001ec05=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec05==(1));
})?({((uuid__000000000001ebfe<(4))?(uuid__000000000001ec07[uuid__000000000001ebfe]=uuid__000000000001ebff):({}));
(uuid__000000000001ec06[uuid__000000000001ebfe]=uuid__000000000001ebff);
}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/umbra.lsts Line: 61 Column: 4"));char rvalue;rvalue;})));
});}
#line 79 "PLATFORM/C/LIB/umbra.lsts"
char* addr_CL_ArrowArray_QM_U8Array_QM_Umbra(LM_Umbra* uuid__000000000001ec08){LM_UmbraShortLong* uuid__000000000001ec09;
LM_UmbraShortLong* uuid__000000000001ec0a;
char uuid__000000000001ec0b;
char* uuid__000000000001ec0c;
LM_UmbraShortLong* uuid__000000000001ec0d;
char uuid__000000000001ec0e;
char* uuid__000000000001ec0f;
return ({({uuid__000000000001ec09=(&uuid__000000000001ec08->field_1);({});})
;
(({({({({uuid__000000000001ec0a=uuid__000000000001ec09;({});})
;
({uuid__000000000001ec0b=(0);({});})
;
});
(((uuid__000000000001ec0a->field_0)==(1))?(({({uuid__000000000001ec0c=(((LM_UmbraShortLong*)(uuid__000000000001ec0a))->field_1001);({});})
;
1;
})?({uuid__000000000001ec0b=(1);({});}):({})):({}));
});
(uuid__000000000001ec0b==(1));
})?((char*)(uuid__000000000001ec0c)):(({({({({uuid__000000000001ec0d=uuid__000000000001ec09;({});})
;
({uuid__000000000001ec0e=(0);({});})
;
});
(((uuid__000000000001ec0d->field_0)==(0))?(({({uuid__000000000001ec0f=(((LM_UmbraShortLong*)(uuid__000000000001ec0d))->field_1);({});})
;
1;
})?({uuid__000000000001ec0e=(1);({});}):({})):({}));
});
(uuid__000000000001ec0e==(1));
})?uuid__000000000001ec0f:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/umbra.lsts Line: 80 Column: 5"));char* rvalue;rvalue;})));
});}
#line 92 "PLATFORM/C/LIB/umbra.lsts"
LM_Umbra view_SB_len_CL_ArrowUmbraConsU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ec10,unsigned long uuid__000000000001ec11){LM_Umbra uuid__000000000001ec12;
unsigned long uuid__000000000001ec13;
return (((uuid__000000000001ec11<=(12))&&((uuid__000000000001ec10->field_2)>(12)))?({({({({uuid__000000000001ec12=(new_SB_umbra_CL_ArrowUmbraU64(uuid__000000000001ec11));({});})
;
({uuid__000000000001ec13=(0);({});})
;
});
({while((uuid__000000000001ec13<uuid__000000000001ec11)){((void)(({(set_LB__RB__CL_ArrowNilConsU8ConsU64Array_QM_Umbra((&uuid__000000000001ec12),uuid__000000000001ec13,(_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec10,uuid__000000000001ec13))));
({uuid__000000000001ec13=(uuid__000000000001ec13+(1));({});});
})));};});
});
uuid__000000000001ec12;
}):({({uuid__000000000001ec10->field_2=((unsigned int)(uuid__000000000001ec11));({});});
(*uuid__000000000001ec10);
}));}
#line 108 "PLATFORM/C/LIB/umbra.lsts"
void print_CL_ArrowNilArray_QM_Umbra(LM_Umbra* uuid__000000000001ec14){unsigned long uuid__000000000001ec15;
char* uuid__000000000001ec16;
({({({uuid__000000000001ec15=(0);({});})
;
({uuid__000000000001ec16=(addr_CL_ArrowArray_QM_U8Array_QM_Umbra(uuid__000000000001ec14));({});})
;
});
({while((uuid__000000000001ec15<(_DT_length_CL_ArrowU64Array_QM_Umbra(uuid__000000000001ec14)))){((void)(({(putchar(((unsigned int)((uuid__000000000001ec16[uuid__000000000001ec15])))));
({uuid__000000000001ec15=(uuid__000000000001ec15+(1));({});});
})));};});
});}
#line 117 "PLATFORM/C/LIB/umbra.lsts"
unsigned long short_SB_prefix_SB_matches_CL_ArrowU64ConsArray_QM_UmbraArray_QM_Umbra(LM_Umbra* uuid__000000000001ec17,LM_Umbra* uuid__000000000001ec18){return (((((_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec17,(0)))==(_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec18,(0))))&&((_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec17,(1)))==(_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec18,(1)))))&&((_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec17,(2)))==(_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec18,(2)))))&&((_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec17,(3)))==(_LB__RB__CL_ArrowU8ConsU64Array_QM_Umbra(uuid__000000000001ec18,(3)))));}
#line 125 "PLATFORM/C/LIB/umbra.lsts"
unsigned long _DT_has_SB_prefix_CL_ArrowU64ConsArray_QM_UmbraArray_QM_Umbra(LM_Umbra* uuid__000000000001ec19,LM_Umbra* uuid__000000000001ec1a){return (((_DT_length_CL_ArrowU64Array_QM_Umbra(uuid__000000000001ec1a))>(_DT_length_CL_ArrowU64Array_QM_Umbra(uuid__000000000001ec19)))?(0):((short_SB_prefix_SB_matches_CL_ArrowU64ConsArray_QM_UmbraArray_QM_Umbra(uuid__000000000001ec19,uuid__000000000001ec1a))&&((memcmp((addr_CL_ArrowArray_QM_U8Array_QM_Umbra(uuid__000000000001ec19)),(addr_CL_ArrowArray_QM_U8Array_QM_Umbra(uuid__000000000001ec1a)),(_DT_length_CL_ArrowU64Array_QM_Umbra(uuid__000000000001ec1a))))==(0))));}
#line 186 "PLATFORM/C/LIB/umbra.lsts"
unsigned long deep_SB_hash_CL_ArrowU64Array_QM_Umbra(LM_Umbra* uuid__000000000001ec1b){unsigned long uuid__000000000001ec1c;
unsigned long uuid__000000000001ec1d;
char* uuid__000000000001ec1e;
return ({({({({({({({({uuid__000000000001ec1c=(0);({});})
;
({uuid__000000000001ec1d=(0);({});})
;
});
({uuid__000000000001ec1e=(addr_CL_ArrowArray_QM_U8Array_QM_Umbra(uuid__000000000001ec1b));({});})
;
});
({while((uuid__000000000001ec1d<(_DT_length_CL_ArrowU64Array_QM_Umbra(uuid__000000000001ec1b)))){((void)(({({({({uuid__000000000001ec1c=(uuid__000000000001ec1c+((unsigned long)((uuid__000000000001ec1e[uuid__000000000001ec1d]))));({});});
({uuid__000000000001ec1c=(uuid__000000000001ec1c+(uuid__000000000001ec1c<<(10)));({});});
});
({uuid__000000000001ec1c=(uuid__000000000001ec1c^(uuid__000000000001ec1c>>(6)));({});});
});
({uuid__000000000001ec1d=(uuid__000000000001ec1d+(1));({});});
})));};});
});
({uuid__000000000001ec1c=(uuid__000000000001ec1c+(uuid__000000000001ec1c<<(3)));({});});
});
({uuid__000000000001ec1c=(uuid__000000000001ec1c^(uuid__000000000001ec1c>>(11)));({});});
});
({uuid__000000000001ec1c=(uuid__000000000001ec1c+(uuid__000000000001ec1c<<(15)));({});});
});
uuid__000000000001ec1c;
});}
#line 202 "PLATFORM/C/LIB/umbra.lsts"
LM_Umbra new_SB_umbra_CL_ArrowUmbraU64(unsigned long uuid__000000000001ec1f){char uuid__000000000001ec20[]={(0),(0),(0),(0),(0),(0),(0),(0),(0),(0),(0),(0),};
char uuid__000000000001ec21[]={(0),(0),(0),(0),};
return ((uuid__000000000001ec1f<=(12))?({;
({LM_Umbra rvalue={.field_0=0};rvalue.field_1=({LM_UmbraShortLong rvalue={.field_0=1};memcpy(&rvalue.field_1001,((char*)(uuid__000000000001ec20)),sizeof(char)*12);rvalue;});rvalue.field_2=((unsigned int)(uuid__000000000001ec1f));rvalue;});
}):({;
({LM_Umbra rvalue={.field_0=0};rvalue.field_1=({LM_UmbraShortLong rvalue={.field_0=0};memcpy(&rvalue.field_1,((char*)((malloc(uuid__000000000001ec1f)))),sizeof(char)*4);memcpy(&rvalue.field_2,((char*)(uuid__000000000001ec21)),sizeof(char)*4);rvalue;});rvalue.field_2=((unsigned int)(uuid__000000000001ec1f));rvalue;});
}));}
#line 6 "SRC/types-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64Type(LM_Type uuid__000000000001ec22){return ((uuid__000000000001ec22.field_0)!=(({LM_Type rvalue={3};rvalue;}).field_0));}
#line 2 "SRC/type-print.lm"
void print_CL_ArrowNilType(LM_Type uuid__000000000001ec23){LM_Type uuid__000000000001ec24;
LM_Type uuid__000000000001ec25;
LM_Type uuid__000000000001ec26;
char uuid__000000000001ec27;
char* uuid__000000000001ec28;
LM_Type uuid__000000000001ec29;
char uuid__000000000001ec2a;
LM_ListType uuid__000000000001ec2b;
char* uuid__000000000001ec2c;
LM_Type uuid__000000000001ec2d;
char uuid__000000000001ec2e;
LM_Type uuid__000000000001ec2f;
LM_Type uuid__000000000001ec30;
LM_Type uuid__000000000001ec31;
char uuid__000000000001ec32;
LM_ListType uuid__000000000001ec33;
char* uuid__000000000001ec34;
({({uuid__000000000001ec24=uuid__000000000001ec23;({});})
;
(({({uuid__000000000001ec25=uuid__000000000001ec24;({});})
;
((uuid__000000000001ec25.field_0)==(3));
})?(print_CL_ArrowNilString("?")):(({({({({uuid__000000000001ec26=uuid__000000000001ec24;({});})
;
({uuid__000000000001ec27=(0);({});})
;
});
(((uuid__000000000001ec26.field_0)==(2))?(({({uuid__000000000001ec28=(((LM_Type)(uuid__000000000001ec26)).field_2001);({});})
;
1;
})?({uuid__000000000001ec27=(1);({});}):({})):({}));
});
(uuid__000000000001ec27==(1));
})?(print_CL_ArrowNilString(uuid__000000000001ec28)):(({({({({uuid__000000000001ec29=uuid__000000000001ec24;({});})
;
({uuid__000000000001ec2a=(0);({});})
;
});
(((uuid__000000000001ec29.field_0)==(1))?(({({uuid__000000000001ec2b=(*(((LM_Type)(uuid__000000000001ec29)).field_1001));({});})
;
((uuid__000000000001ec2b.field_0)==(1));
})?(({({uuid__000000000001ec2c=(((LM_Type)(uuid__000000000001ec29)).field_1002);({});})
;
1;
})?({uuid__000000000001ec2a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec2a==(1));
})?(print_CL_ArrowNilString(uuid__000000000001ec2c)):(({({({({uuid__000000000001ec2d=uuid__000000000001ec24;({});})
;
({uuid__000000000001ec2e=(0);({});})
;
});
(((uuid__000000000001ec2d.field_0)==(0))?(({({uuid__000000000001ec2f=(*(((LM_Type)(uuid__000000000001ec2d)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec30=(*(((LM_Type)(uuid__000000000001ec2d)).field_2));({});})
;
1;
})?({uuid__000000000001ec2e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec2e==(1));
})?((_DT_is_SB_arrow_CL_ArrowU64Type(uuid__000000000001ec30))?({({(print_CL_ArrowNilType(uuid__000000000001ec30));
(print_CL_ArrowNilString(" +\n"));
});
(print_CL_ArrowNilType(uuid__000000000001ec2f));
}):({({(print_CL_ArrowNilType(uuid__000000000001ec30));
(print_CL_ArrowNilString(" + "));
});
(print_CL_ArrowNilType(uuid__000000000001ec2f));
})):(({({({({uuid__000000000001ec31=uuid__000000000001ec24;({});})
;
({uuid__000000000001ec32=(0);({});})
;
});
(((uuid__000000000001ec31.field_0)==(1))?(({({uuid__000000000001ec33=(*(((LM_Type)(uuid__000000000001ec31)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ec34=(((LM_Type)(uuid__000000000001ec31)).field_1002);({});})
;
1;
})?({uuid__000000000001ec32=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec32==(1));
})?({({({(print_CL_ArrowNilString(uuid__000000000001ec34));
(print_CL_ArrowNilString("<"));
});
(print_CL_ArrowNilListType(uuid__000000000001ec33));
});
(print_CL_ArrowNilString(">"));
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/type-print.lm Line: 2 Column: 29")))))));
});}
#line 26 "SRC/type-print.lm"
void print_CL_ArrowNilListType(LM_ListType uuid__000000000001ec35){LM_ListType uuid__000000000001ec36;
LM_ListType uuid__000000000001ec37;
LM_ListType uuid__000000000001ec38;
char uuid__000000000001ec39;
LM_ListType uuid__000000000001ec3a;
LM_Type uuid__000000000001ec3b;
LM_ListType uuid__000000000001ec3c;
char uuid__000000000001ec3d;
LM_ListType uuid__000000000001ec3e;
LM_Type uuid__000000000001ec3f;
({({uuid__000000000001ec36=uuid__000000000001ec35;({});})
;
(({({uuid__000000000001ec37=uuid__000000000001ec36;({});})
;
((uuid__000000000001ec37.field_0)==(1));
})?({}):(({({({({uuid__000000000001ec38=uuid__000000000001ec36;({});})
;
({uuid__000000000001ec39=(0);({});})
;
});
(((uuid__000000000001ec38.field_0)==(0))?(({({uuid__000000000001ec3a=(*(((LM_ListType)(uuid__000000000001ec38)).field_1));({});})
;
((uuid__000000000001ec3a.field_0)==(1));
})?(({({uuid__000000000001ec3b=(((LM_ListType)(uuid__000000000001ec38)).field_2);({});})
;
1;
})?({uuid__000000000001ec39=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec39==(1));
})?(print_CL_ArrowNilType(uuid__000000000001ec3b)):(({({({({uuid__000000000001ec3c=uuid__000000000001ec36;({});})
;
({uuid__000000000001ec3d=(0);({});})
;
});
(((uuid__000000000001ec3c.field_0)==(0))?(({({uuid__000000000001ec3e=(*(((LM_ListType)(uuid__000000000001ec3c)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec3f=(((LM_ListType)(uuid__000000000001ec3c)).field_2);({});})
;
1;
})?({uuid__000000000001ec3d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec3d==(1));
})?({({(print_CL_ArrowNilListType(uuid__000000000001ec3e));
(print_CL_ArrowNilString(","));
});
(print_CL_ArrowNilType(uuid__000000000001ec3f));
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/type-print.lm Line: 26 Column: 35")))));
});}
#line 2 "SRC/t.lsts"
LM_Type t1_CL_ArrowTypeString(char* uuid__000000000001ec40){return ({LM_Type rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_ListTypeListType(({LM_ListType rvalue={1};rvalue;})));rvalue.field_1002=uuid__000000000001ec40;rvalue;});}
#line 3 "SRC/t.lsts"
LM_Type t2_CL_ArrowTypeConsTypeString(char* uuid__000000000001ec42,LM_Type uuid__000000000001ec43){return ({LM_Type rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_ListTypeListType((list_CL__CL_cons_CL_ArrowListTypeConsListTypeType(uuid__000000000001ec43,({LM_ListType rvalue={1};rvalue;})))));rvalue.field_1002=uuid__000000000001ec42;rvalue;});}
#line 4 "SRC/t.lsts"
LM_Type t3_CL_ArrowTypeConsTypeConsTypeString(char* uuid__000000000001ec45,LM_Type uuid__000000000001ec46,LM_Type uuid__000000000001ec47){return ({LM_Type rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_ListTypeListType((list_CL__CL_cons_CL_ArrowListTypeConsListTypeType(uuid__000000000001ec47,(list_CL__CL_cons_CL_ArrowListTypeConsListTypeType(uuid__000000000001ec46,({LM_ListType rvalue={1};rvalue;})))))));rvalue.field_1002=uuid__000000000001ec45;rvalue;});}
#line 5 "SRC/t.lsts"
LM_Type tv_CL_ArrowTypeString(char* uuid__000000000001ec49){return ({LM_Type rvalue={.field_0=2};rvalue.field_2001=uuid__000000000001ec49;rvalue;});}
#line 6 "SRC/t.lsts"
LM_Type _AM__AM__CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001ec4a,LM_Type uuid__000000000001ec4b){return ((!(non_SB_zero_CL_ArrowU64Type(uuid__000000000001ec4a)))?uuid__000000000001ec4b:((!(non_SB_zero_CL_ArrowU64Type(uuid__000000000001ec4b)))?uuid__000000000001ec4a:({LM_Type rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001ec4b));rvalue.field_2=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001ec4a));rvalue;})));}
#line 7 "SRC/t.lsts"
LM_Type _BR__BR__CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001ec4c,LM_Type uuid__000000000001ec4d){return ((non_SB_zero_CL_ArrowU64Type(uuid__000000000001ec4c))?uuid__000000000001ec4c:uuid__000000000001ec4d);}
#line 2 "SRC/p.lsts"
LM_Type _DT_p_CL_ArrowTypeConsU64Type(LM_Type uuid__000000000001ec4e,unsigned long uuid__000000000001ec4f){LM_Type uuid__000000000001ec50;
LM_Type uuid__000000000001ec51;
char uuid__000000000001ec52;
LM_ListType uuid__000000000001ec53;
return ({({uuid__000000000001ec50=uuid__000000000001ec4e;({});})
;
(({({({({uuid__000000000001ec51=uuid__000000000001ec50;({});})
;
({uuid__000000000001ec52=(0);({});})
;
});
(((uuid__000000000001ec51.field_0)==(1))?(({({uuid__000000000001ec53=(*(((LM_Type)(uuid__000000000001ec51)).field_1001));({});})
;
1;
})?({uuid__000000000001ec52=(1);({});}):({})):({}));
});
(uuid__000000000001ec52==(1));
})?(_DT_nth_CL_ArrowTypeConsTypeConsU64ListType(uuid__000000000001ec53,uuid__000000000001ec4f,({LM_Type rvalue={3};rvalue;}))):(1?({LM_Type rvalue={3};rvalue;}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/p.lsts Line: 3 Column: 4"));LM_Type rvalue;rvalue;})));
});}
#line 8 "SRC/p.lsts"
LM_Type _DT_p1_CL_ArrowTypeType(LM_Type uuid__000000000001ec54){return (_DT_p_CL_ArrowTypeConsU64Type(uuid__000000000001ec54,(0)));}
#line 9 "SRC/p.lsts"
LM_Type _DT_p2_CL_ArrowTypeType(LM_Type uuid__000000000001ec55){return (_DT_p_CL_ArrowTypeConsU64Type(uuid__000000000001ec55,(1)));}
#line 2 "SRC/range.lsts"
LM_Type _DT_range_CL_ArrowTypeType(LM_Type uuid__000000000001ec56){LM_Type uuid__000000000001ec57;
LM_Type uuid__000000000001ec58;
char uuid__000000000001ec59;
LM_Type uuid__000000000001ec5a;
LM_Type uuid__000000000001ec5b;
LM_Type uuid__000000000001ec5c;
char uuid__000000000001ec5d;
LM_ListType uuid__000000000001ec5e;
LM_Type uuid__000000000001ec5f;
LM_ListType uuid__000000000001ec60;
LM_Type uuid__000000000001ec61;
LM_ListType uuid__000000000001ec62;
char* uuid__000000000001ec63;
LM_Type uuid__000000000001ec64;
char uuid__000000000001ec65;
LM_ListType uuid__000000000001ec66;
LM_ListType uuid__000000000001ec67;
LM_Type uuid__000000000001ec68;
LM_ListType uuid__000000000001ec69;
char* uuid__000000000001ec6a;
return ({({uuid__000000000001ec57=uuid__000000000001ec56;({});})
;
(({({({({uuid__000000000001ec58=uuid__000000000001ec57;({});})
;
({uuid__000000000001ec59=(0);({});})
;
});
(((uuid__000000000001ec58.field_0)==(0))?(({({uuid__000000000001ec5a=(*(((LM_Type)(uuid__000000000001ec58)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec5b=(*(((LM_Type)(uuid__000000000001ec58)).field_2));({});})
;
1;
})?({uuid__000000000001ec59=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec59==(1));
})?(_BR__BR__CL_ArrowTypeConsTypeType((_DT_range_CL_ArrowTypeType(uuid__000000000001ec5b)),(_DT_range_CL_ArrowTypeType(uuid__000000000001ec5a)))):(({({({({uuid__000000000001ec5c=uuid__000000000001ec57;({});})
;
({uuid__000000000001ec5d=(0);({});})
;
});
(((uuid__000000000001ec5c.field_0)==(1))?(({({uuid__000000000001ec5e=(*(((LM_Type)(uuid__000000000001ec5c)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec5e))?(({({uuid__000000000001ec5f=(head_CL_ArrowTypeListType(uuid__000000000001ec5e));({});})
;
1;
})?({({uuid__000000000001ec60=(tail_CL_ArrowListTypeListType(uuid__000000000001ec5e));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec60))?(({({uuid__000000000001ec61=(head_CL_ArrowTypeListType(uuid__000000000001ec60));({});})
;
1;
})?({({uuid__000000000001ec62=(tail_CL_ArrowListTypeListType(uuid__000000000001ec60));({});})
;
((uuid__000000000001ec62.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ec63=(((LM_Type)(uuid__000000000001ec5c)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec63,"Arrow"));
})?({uuid__000000000001ec5d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec5d==(1));
})?uuid__000000000001ec5f:(({({({({uuid__000000000001ec64=uuid__000000000001ec57;({});})
;
({uuid__000000000001ec65=(0);({});})
;
});
(((uuid__000000000001ec64.field_0)==(1))?(({({uuid__000000000001ec66=(*(((LM_Type)(uuid__000000000001ec64)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec66))?(1?({({uuid__000000000001ec67=(tail_CL_ArrowListTypeListType(uuid__000000000001ec66));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec67))?(({({uuid__000000000001ec68=(head_CL_ArrowTypeListType(uuid__000000000001ec67));({});})
;
1;
})?({({uuid__000000000001ec69=(tail_CL_ArrowListTypeListType(uuid__000000000001ec67));({});})
;
((uuid__000000000001ec69.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ec6a=(((LM_Type)(uuid__000000000001ec64)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec6a,"Array"));
})?({uuid__000000000001ec65=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec65==(1));
})?(_DT_range_CL_ArrowTypeType(uuid__000000000001ec68)):(1?({LM_Type rvalue={3};rvalue;}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/range.lsts Line: 3 Column: 4"));LM_Type rvalue;rvalue;})))));
});}
#line 2 "SRC/domain.lsts"
LM_Type _DT_domain_CL_ArrowTypeType(LM_Type uuid__000000000001ec6b){LM_Type uuid__000000000001ec6c;
LM_Type uuid__000000000001ec6d;
char uuid__000000000001ec6e;
LM_Type uuid__000000000001ec6f;
LM_Type uuid__000000000001ec70;
LM_Type uuid__000000000001ec71;
char uuid__000000000001ec72;
LM_ListType uuid__000000000001ec73;
LM_Type uuid__000000000001ec74;
LM_ListType uuid__000000000001ec75;
LM_Type uuid__000000000001ec76;
LM_ListType uuid__000000000001ec77;
char* uuid__000000000001ec78;
LM_Type uuid__000000000001ec79;
char uuid__000000000001ec7a;
LM_ListType uuid__000000000001ec7b;
LM_ListType uuid__000000000001ec7c;
LM_Type uuid__000000000001ec7d;
LM_ListType uuid__000000000001ec7e;
char* uuid__000000000001ec7f;
return ({({uuid__000000000001ec6c=uuid__000000000001ec6b;({});})
;
(({({({({uuid__000000000001ec6d=uuid__000000000001ec6c;({});})
;
({uuid__000000000001ec6e=(0);({});})
;
});
(((uuid__000000000001ec6d.field_0)==(0))?(({({uuid__000000000001ec6f=(*(((LM_Type)(uuid__000000000001ec6d)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec70=(*(((LM_Type)(uuid__000000000001ec6d)).field_2));({});})
;
1;
})?({uuid__000000000001ec6e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec6e==(1));
})?(_BR__BR__CL_ArrowTypeConsTypeType((_DT_domain_CL_ArrowTypeType(uuid__000000000001ec70)),(_DT_domain_CL_ArrowTypeType(uuid__000000000001ec6f)))):(({({({({uuid__000000000001ec71=uuid__000000000001ec6c;({});})
;
({uuid__000000000001ec72=(0);({});})
;
});
(((uuid__000000000001ec71.field_0)==(1))?(({({uuid__000000000001ec73=(*(((LM_Type)(uuid__000000000001ec71)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec73))?(({({uuid__000000000001ec74=(head_CL_ArrowTypeListType(uuid__000000000001ec73));({});})
;
1;
})?({({uuid__000000000001ec75=(tail_CL_ArrowListTypeListType(uuid__000000000001ec73));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec75))?(({({uuid__000000000001ec76=(head_CL_ArrowTypeListType(uuid__000000000001ec75));({});})
;
1;
})?({({uuid__000000000001ec77=(tail_CL_ArrowListTypeListType(uuid__000000000001ec75));({});})
;
((uuid__000000000001ec77.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ec78=(((LM_Type)(uuid__000000000001ec71)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec78,"Arrow"));
})?({uuid__000000000001ec72=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec72==(1));
})?uuid__000000000001ec76:(({({({({uuid__000000000001ec79=uuid__000000000001ec6c;({});})
;
({uuid__000000000001ec7a=(0);({});})
;
});
(((uuid__000000000001ec79.field_0)==(1))?(({({uuid__000000000001ec7b=(*(((LM_Type)(uuid__000000000001ec79)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec7b))?(1?({({uuid__000000000001ec7c=(tail_CL_ArrowListTypeListType(uuid__000000000001ec7b));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ec7c))?(({({uuid__000000000001ec7d=(head_CL_ArrowTypeListType(uuid__000000000001ec7c));({});})
;
1;
})?({({uuid__000000000001ec7e=(tail_CL_ArrowListTypeListType(uuid__000000000001ec7c));({});})
;
((uuid__000000000001ec7e.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ec7f=(((LM_Type)(uuid__000000000001ec79)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec7f,"Array"));
})?({uuid__000000000001ec7a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec7a==(1));
})?(_DT_domain_CL_ArrowTypeType(uuid__000000000001ec7d)):(1?({LM_Type rvalue={3};rvalue;}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/domain.lsts Line: 3 Column: 4"));LM_Type rvalue;rvalue;})))));
});}
#line 2 "SRC/arity.lsts"
unsigned long _DT_arity_CL_ArrowU64Type(LM_Type uuid__000000000001ec80){LM_Type uuid__000000000001ec81;
LM_Type uuid__000000000001ec82;
char uuid__000000000001ec83;
LM_ListType uuid__000000000001ec84;
return ({({uuid__000000000001ec81=uuid__000000000001ec80;({});})
;
(({({({({uuid__000000000001ec82=uuid__000000000001ec81;({});})
;
({uuid__000000000001ec83=(0);({});})
;
});
(((uuid__000000000001ec82.field_0)==(1))?(({({uuid__000000000001ec84=(*(((LM_Type)(uuid__000000000001ec82)).field_1001));({});})
;
1;
})?({uuid__000000000001ec83=(1);({});}):({})):({}));
});
(uuid__000000000001ec83==(1));
})?(_DT_length_CL_ArrowU64ListType(uuid__000000000001ec84)):(1?(0):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/arity.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;})));
});}
#line 2 "SRC/slot.lsts"
LM_Type _DT_slot_CL_ArrowTypeConsStringType(LM_Type uuid__000000000001ec85,char* uuid__000000000001ec86){LM_Type uuid__000000000001ec87;
LM_Type uuid__000000000001ec88;
char uuid__000000000001ec89;
char* uuid__000000000001ec8a;
LM_Type uuid__000000000001ec8b;
char uuid__000000000001ec8c;
LM_Type uuid__000000000001ec8d;
LM_Type uuid__000000000001ec8e;
LM_Type uuid__000000000001ec8f;
return ({({uuid__000000000001ec87=uuid__000000000001ec85;({});})
;
(({({({({uuid__000000000001ec88=uuid__000000000001ec87;({});})
;
({uuid__000000000001ec89=(0);({});})
;
});
(((uuid__000000000001ec88.field_0)==(1))?(({({uuid__000000000001ec8a=(((LM_Type)(uuid__000000000001ec88)).field_1002);({});})
;
1;
})?({uuid__000000000001ec89=(1);({});}):({})):({}));
});
(uuid__000000000001ec89==(1));
})?((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec8a,uuid__000000000001ec86))?uuid__000000000001ec85:({LM_Type rvalue={3};rvalue;})):(({({({({uuid__000000000001ec8b=uuid__000000000001ec87;({});})
;
({uuid__000000000001ec8c=(0);({});})
;
});
(((uuid__000000000001ec8b.field_0)==(0))?(({({uuid__000000000001ec8d=(*(((LM_Type)(uuid__000000000001ec8b)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec8e=(*(((LM_Type)(uuid__000000000001ec8b)).field_2));({});})
;
1;
})?({uuid__000000000001ec8c=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec8c==(1));
})?({({uuid__000000000001ec8f=(_DT_slot_CL_ArrowTypeConsStringType(uuid__000000000001ec8e,uuid__000000000001ec86));({});})
;
((non_SB_zero_CL_ArrowU64Type(uuid__000000000001ec8f))?uuid__000000000001ec8f:(_DT_slot_CL_ArrowTypeConsStringType(uuid__000000000001ec8d,uuid__000000000001ec86)));
}):(1?({LM_Type rvalue={3};rvalue;}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/slot.lsts Line: 3 Column: 4"));LM_Type rvalue;rvalue;}))));
});}
#line 2 "SRC/is-t.lsts"
unsigned long _DT_is_SB_t_CL_ArrowU64ConsStringType(LM_Type uuid__000000000001ec90,char* uuid__000000000001ec91){LM_Type uuid__000000000001ec92;
LM_Type uuid__000000000001ec93;
char uuid__000000000001ec94;
LM_Type uuid__000000000001ec95;
LM_Type uuid__000000000001ec96;
LM_Type uuid__000000000001ec97;
char uuid__000000000001ec98;
char* uuid__000000000001ec99;
return ({({uuid__000000000001ec92=uuid__000000000001ec90;({});})
;
(({({({({uuid__000000000001ec93=uuid__000000000001ec92;({});})
;
({uuid__000000000001ec94=(0);({});})
;
});
(((uuid__000000000001ec93.field_0)==(0))?(({({uuid__000000000001ec95=(*(((LM_Type)(uuid__000000000001ec93)).field_1));({});})
;
1;
})?(({({uuid__000000000001ec96=(*(((LM_Type)(uuid__000000000001ec93)).field_2));({});})
;
1;
})?({uuid__000000000001ec94=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec94==(1));
})?((_DT_is_SB_t_CL_ArrowU64ConsStringType(uuid__000000000001ec96,uuid__000000000001ec91))||(_DT_is_SB_t_CL_ArrowU64ConsStringType(uuid__000000000001ec95,uuid__000000000001ec91))):(({({({({uuid__000000000001ec97=uuid__000000000001ec92;({});})
;
({uuid__000000000001ec98=(0);({});})
;
});
(((uuid__000000000001ec97.field_0)==(1))?(({({uuid__000000000001ec99=(((LM_Type)(uuid__000000000001ec97)).field_1002);({});})
;
1;
})?({uuid__000000000001ec98=(1);({});}):({})):({}));
});
(uuid__000000000001ec98==(1));
})?(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ec99,uuid__000000000001ec91)):(1?false_CL_U64:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/is-t.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;}))));
});}
#line 2 "SRC/is-open.lsts"
unsigned long _DT_is_SB_open_CL_ArrowU64Type(LM_Type uuid__000000000001ec9a){LM_Type uuid__000000000001ec9b;
LM_Type uuid__000000000001ec9c;
LM_Type uuid__000000000001ec9d;
LM_Type uuid__000000000001ec9e;
char uuid__000000000001ec9f;
LM_Type uuid__000000000001eca0;
LM_Type uuid__000000000001eca1;
LM_Type uuid__000000000001eca2;
char uuid__000000000001eca3;
LM_ListType uuid__000000000001eca4;
LM_ListType uuid__000000000001eca5;
LM_Type uuid__000000000001eca6;
LM_ListType uuid__000000000001eca7;
char* uuid__000000000001eca8;
LM_Type uuid__000000000001eca9;
char uuid__000000000001ecaa;
LM_ListType uuid__000000000001ecab;
LM_ListType uuid__000000000001ecac;
LM_Type uuid__000000000001ecad;
LM_ListType uuid__000000000001ecae;
char* uuid__000000000001ecaf;
LM_Type uuid__000000000001ecb0;
char uuid__000000000001ecb1;
LM_ListType uuid__000000000001ecb2;
unsigned long uuid__000000000001ecb3;
LM_ListType uuid__000000000001ecb4;
LM_ListType uuid__000000000001ecb5;
LM_ListType uuid__000000000001ecb6;
char uuid__000000000001ecb7;
LM_ListType uuid__000000000001ecb8;
LM_Type uuid__000000000001ecb9;
return ({({uuid__000000000001ec9b=uuid__000000000001ec9a;({});})
;
(({({uuid__000000000001ec9c=uuid__000000000001ec9b;({});})
;
((uuid__000000000001ec9c.field_0)==(3));
})?true_CL_U64:(({({uuid__000000000001ec9d=uuid__000000000001ec9b;({});})
;
((uuid__000000000001ec9d.field_0)==(2));
})?true_CL_U64:(({({({({uuid__000000000001ec9e=uuid__000000000001ec9b;({});})
;
({uuid__000000000001ec9f=(0);({});})
;
});
(((uuid__000000000001ec9e.field_0)==(0))?(({({uuid__000000000001eca0=(*(((LM_Type)(uuid__000000000001ec9e)).field_1));({});})
;
1;
})?(({({uuid__000000000001eca1=(*(((LM_Type)(uuid__000000000001ec9e)).field_2));({});})
;
1;
})?({uuid__000000000001ec9f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ec9f==(1));
})?((_DT_is_SB_open_CL_ArrowU64Type(uuid__000000000001eca1))||(_DT_is_SB_open_CL_ArrowU64Type(uuid__000000000001eca0))):(({({({({uuid__000000000001eca2=uuid__000000000001ec9b;({});})
;
({uuid__000000000001eca3=(0);({});})
;
});
(((uuid__000000000001eca2.field_0)==(1))?(({({uuid__000000000001eca4=(*(((LM_Type)(uuid__000000000001eca2)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001eca4))?(1?({({uuid__000000000001eca5=(tail_CL_ArrowListTypeListType(uuid__000000000001eca4));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001eca5))?(({({uuid__000000000001eca6=(head_CL_ArrowTypeListType(uuid__000000000001eca5));({});})
;
((uuid__000000000001eca6.field_0)==(3));
})?({({uuid__000000000001eca7=(tail_CL_ArrowListTypeListType(uuid__000000000001eca5));({});})
;
((uuid__000000000001eca7.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001eca8=(((LM_Type)(uuid__000000000001eca2)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eca8,"Array"));
})?({uuid__000000000001eca3=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eca3==(1));
})?false_CL_U64:(({({({({uuid__000000000001eca9=uuid__000000000001ec9b;({});})
;
({uuid__000000000001ecaa=(0);({});})
;
});
(((uuid__000000000001eca9.field_0)==(1))?(({({uuid__000000000001ecab=(*(((LM_Type)(uuid__000000000001eca9)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ecab))?(1?({({uuid__000000000001ecac=(tail_CL_ArrowListTypeListType(uuid__000000000001ecab));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ecac))?(({({uuid__000000000001ecad=(head_CL_ArrowTypeListType(uuid__000000000001ecac));({});})
;
1;
})?({({uuid__000000000001ecae=(tail_CL_ArrowListTypeListType(uuid__000000000001ecac));({});})
;
((uuid__000000000001ecae.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ecaf=(((LM_Type)(uuid__000000000001eca9)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecaf,"Array"));
})?({uuid__000000000001ecaa=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecaa==(1));
})?(_DT_is_SB_open_CL_ArrowU64Type(uuid__000000000001ecad)):(({({({({uuid__000000000001ecb0=uuid__000000000001ec9b;({});})
;
({uuid__000000000001ecb1=(0);({});})
;
});
(((uuid__000000000001ecb0.field_0)==(1))?(({({uuid__000000000001ecb2=(*(((LM_Type)(uuid__000000000001ecb0)).field_1001));({});})
;
1;
})?({uuid__000000000001ecb1=(1);({});}):({})):({}));
});
(uuid__000000000001ecb1==(1));
})?({({({uuid__000000000001ecb3=false_CL_U64;({});})
;
({({uuid__000000000001ecb4=uuid__000000000001ecb2;({});})
;
({while((non_SB_zero_CL_ArrowU64ListType(uuid__000000000001ecb4))){((void)(({({uuid__000000000001ecb5=uuid__000000000001ecb4;({});})
;
(({({({({uuid__000000000001ecb6=uuid__000000000001ecb5;({});})
;
({uuid__000000000001ecb7=(0);({});})
;
});
(((uuid__000000000001ecb6.field_0)==(0))?(({({uuid__000000000001ecb8=(*(((LM_ListType)(uuid__000000000001ecb6)).field_1));({});})
;
1;
})?(({({uuid__000000000001ecb9=(((LM_ListType)(uuid__000000000001ecb6)).field_2);({});})
;
1;
})?({uuid__000000000001ecb7=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecb7==(1));
})?({({uuid__000000000001ecb3=(uuid__000000000001ecb3||(_DT_is_SB_open_CL_ArrowU64Type(uuid__000000000001ecb9)));({});});
({uuid__000000000001ecb4=uuid__000000000001ecb8;({});});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/is-open.lsts Line: 11 Column: 10")));
})));};});
});
});
uuid__000000000001ecb3;
}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/is-open.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;})))))));
});}
#line 2 "SRC/is-arrow.lsts"
unsigned long _DT_is_SB_arrow_CL_ArrowU64Type(LM_Type uuid__000000000001ecba){LM_Type uuid__000000000001ecbb;
LM_Type uuid__000000000001ecbc;
char uuid__000000000001ecbd;
LM_Type uuid__000000000001ecbe;
LM_Type uuid__000000000001ecbf;
LM_Type uuid__000000000001ecc0;
char uuid__000000000001ecc1;
char* uuid__000000000001ecc2;
LM_Type uuid__000000000001ecc3;
char uuid__000000000001ecc4;
LM_ListType uuid__000000000001ecc5;
LM_ListType uuid__000000000001ecc6;
LM_Type uuid__000000000001ecc7;
LM_ListType uuid__000000000001ecc8;
char* uuid__000000000001ecc9;
return ({({uuid__000000000001ecbb=uuid__000000000001ecba;({});})
;
(({({({({uuid__000000000001ecbc=uuid__000000000001ecbb;({});})
;
({uuid__000000000001ecbd=(0);({});})
;
});
(((uuid__000000000001ecbc.field_0)==(0))?(({({uuid__000000000001ecbe=(*(((LM_Type)(uuid__000000000001ecbc)).field_1));({});})
;
1;
})?(({({uuid__000000000001ecbf=(*(((LM_Type)(uuid__000000000001ecbc)).field_2));({});})
;
1;
})?({uuid__000000000001ecbd=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecbd==(1));
})?((_DT_is_SB_arrow_CL_ArrowU64Type(uuid__000000000001ecbf))||(_DT_is_SB_arrow_CL_ArrowU64Type(uuid__000000000001ecbe))):(({({({({uuid__000000000001ecc0=uuid__000000000001ecbb;({});})
;
({uuid__000000000001ecc1=(0);({});})
;
});
(((uuid__000000000001ecc0.field_0)==(1))?(({({uuid__000000000001ecc2=(((LM_Type)(uuid__000000000001ecc0)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecc2,"Arrow"));
})?({uuid__000000000001ecc1=(1);({});}):({})):({}));
});
(uuid__000000000001ecc1==(1));
})?true_CL_U64:(({({({({uuid__000000000001ecc3=uuid__000000000001ecbb;({});})
;
({uuid__000000000001ecc4=(0);({});})
;
});
(((uuid__000000000001ecc3.field_0)==(1))?(({({uuid__000000000001ecc5=(*(((LM_Type)(uuid__000000000001ecc3)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ecc5))?(1?({({uuid__000000000001ecc6=(tail_CL_ArrowListTypeListType(uuid__000000000001ecc5));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ecc6))?(({({uuid__000000000001ecc7=(head_CL_ArrowTypeListType(uuid__000000000001ecc6));({});})
;
1;
})?({({uuid__000000000001ecc8=(tail_CL_ArrowListTypeListType(uuid__000000000001ecc6));({});})
;
((uuid__000000000001ecc8.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ecc9=(((LM_Type)(uuid__000000000001ecc3)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecc9,"Array"));
})?({uuid__000000000001ecc4=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecc4==(1));
})?(_DT_is_SB_arrow_CL_ArrowU64Type(uuid__000000000001ecc7)):(1?false_CL_U64:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/is-arrow.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;})))));
});}
#line 2 "SRC/has-class.lsts"
unsigned long _DT_has_SB_class_CL_ArrowU64Type(LM_Type uuid__000000000001ecca){LM_Type uuid__000000000001eccb;
LM_Type uuid__000000000001eccc;
LM_Type uuid__000000000001eccd;
char uuid__000000000001ecce;
LM_Type uuid__000000000001eccf;
LM_Type uuid__000000000001ecd0;
return ({({uuid__000000000001eccb=uuid__000000000001ecca;({});})
;
(({({uuid__000000000001eccc=uuid__000000000001eccb;({});})
;
((uuid__000000000001eccc.field_0)==(1));
})?true_CL_U64:(({({({({uuid__000000000001eccd=uuid__000000000001eccb;({});})
;
({uuid__000000000001ecce=(0);({});})
;
});
(((uuid__000000000001eccd.field_0)==(0))?(({({uuid__000000000001eccf=(*(((LM_Type)(uuid__000000000001eccd)).field_1));({});})
;
1;
})?(({({uuid__000000000001ecd0=(*(((LM_Type)(uuid__000000000001eccd)).field_2));({});})
;
1;
})?({uuid__000000000001ecce=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecce==(1));
})?((_DT_has_SB_class_CL_ArrowU64Type(uuid__000000000001ecd0))||(_DT_has_SB_class_CL_ArrowU64Type(uuid__000000000001eccf))):(1?false_CL_U64:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/has-class.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;}))));
});}
#line 3 "SRC/without-tag.lsts"
LM_Type _DT_without_SB_tag_CL_ArrowTypeType(LM_Type uuid__000000000001ecd1){LM_Type uuid__000000000001ecd2;
LM_Type uuid__000000000001ecd3;
char uuid__000000000001ecd4;
LM_Type uuid__000000000001ecd5;
LM_Type uuid__000000000001ecd6;
LM_Type uuid__000000000001ecd7;
char uuid__000000000001ecd8;
char* uuid__000000000001ecd9;
LM_Type uuid__000000000001ecda;
char uuid__000000000001ecdb;
char* uuid__000000000001ecdc;
LM_Type uuid__000000000001ecdd;
char uuid__000000000001ecde;
char* uuid__000000000001ecdf;
LM_Type uuid__000000000001ece0;
char uuid__000000000001ece1;
char* uuid__000000000001ece2;
return ({({uuid__000000000001ecd2=uuid__000000000001ecd1;({});})
;
(({({({({uuid__000000000001ecd3=uuid__000000000001ecd2;({});})
;
({uuid__000000000001ecd4=(0);({});})
;
});
(((uuid__000000000001ecd3.field_0)==(0))?(({({uuid__000000000001ecd5=(*(((LM_Type)(uuid__000000000001ecd3)).field_1));({});})
;
1;
})?(({({uuid__000000000001ecd6=(*(((LM_Type)(uuid__000000000001ecd3)).field_2));({});})
;
1;
})?({uuid__000000000001ecd4=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecd4==(1));
})?(_AM__AM__CL_ArrowTypeConsTypeType((_DT_without_SB_tag_CL_ArrowTypeType(uuid__000000000001ecd6)),(_DT_without_SB_tag_CL_ArrowTypeType(uuid__000000000001ecd5)))):(({({({({uuid__000000000001ecd7=uuid__000000000001ecd2;({});})
;
({uuid__000000000001ecd8=(0);({});})
;
});
(((uuid__000000000001ecd7.field_0)==(1))?(({({uuid__000000000001ecd9=(((LM_Type)(uuid__000000000001ecd7)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecd9,"Constructor"));
})?({uuid__000000000001ecd8=(1);({});}):({})):({}));
});
(uuid__000000000001ecd8==(1));
})?({LM_Type rvalue={3};rvalue;}):(({({({({uuid__000000000001ecda=uuid__000000000001ecd2;({});})
;
({uuid__000000000001ecdb=(0);({});})
;
});
(((uuid__000000000001ecda.field_0)==(1))?(({({uuid__000000000001ecdc=(((LM_Type)(uuid__000000000001ecda)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecdc,"CaseNumber"));
})?({uuid__000000000001ecdb=(1);({});}):({})):({}));
});
(uuid__000000000001ecdb==(1));
})?({LM_Type rvalue={3};rvalue;}):(({({({({uuid__000000000001ecdd=uuid__000000000001ecd2;({});})
;
({uuid__000000000001ecde=(0);({});})
;
});
(((uuid__000000000001ecdd.field_0)==(1))?(({({uuid__000000000001ecdf=(((LM_Type)(uuid__000000000001ecdd)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ecdf,"Raw"));
})?({uuid__000000000001ecde=(1);({});}):({})):({}));
});
(uuid__000000000001ecde==(1));
})?({LM_Type rvalue={3};rvalue;}):(({({({({uuid__000000000001ece0=uuid__000000000001ecd2;({});})
;
({uuid__000000000001ece1=(0);({});})
;
});
(((uuid__000000000001ece0.field_0)==(1))?(({({uuid__000000000001ece2=(((LM_Type)(uuid__000000000001ece0)).field_1002);({});})
;
1;
})?({uuid__000000000001ece1=(1);({});}):({})):({}));
});
(uuid__000000000001ece1==(1));
})?((_DT_has_SB_prefix_CL_ArrowU64ConsStringString(uuid__000000000001ece2,"Tag::"))?({LM_Type rvalue={3};rvalue;}):uuid__000000000001ecd1):(1?uuid__000000000001ecd1:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/without-tag.lsts Line: 4 Column: 4"));LM_Type rvalue;rvalue;})))))));
});}
#line 14 "SRC/without-tag.lsts"
LM_ListType _DT_without_SB_tag_CL_ArrowListTypeListType(LM_ListType uuid__000000000001ece3){LM_ListType uuid__000000000001ece4;
LM_ListType uuid__000000000001ece5;
LM_Type uuid__000000000001ece6;
LM_ListType uuid__000000000001ece7;
LM_ListType uuid__000000000001ece8;
return ({({uuid__000000000001ece4=uuid__000000000001ece3;({});})
;
(({({uuid__000000000001ece5=uuid__000000000001ece4;({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ece5))?(({({uuid__000000000001ece6=(head_CL_ArrowTypeListType(uuid__000000000001ece5));({});})
;
1;
})?({({uuid__000000000001ece7=(tail_CL_ArrowListTypeListType(uuid__000000000001ece5));({});})
;
1;
}):0):0);
})?(cons_CL_ArrowListTypeConsListTypeType((_DT_without_SB_tag_CL_ArrowTypeType(uuid__000000000001ece6)),(_DT_without_SB_tag_CL_ArrowListTypeListType(uuid__000000000001ece7)))):(({({uuid__000000000001ece8=uuid__000000000001ece4;({});})
;
1;
})?uuid__000000000001ece8:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/without-tag.lsts Line: 15 Column: 4"));LM_ListType rvalue;rvalue;})));
});}
#line 16 "SRC/ast-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64AST(LM_AST uuid__000000000001ece9){return ((uuid__000000000001ece9.field_0)!=(({LM_AST rvalue={10};rvalue;}).field_0));}
#line 19 "SRC/ast-definitions.lsts"
LM_AST App_CL_ArrowASTConsArray_QM_ASTArray_QM_AST(LM_AST* uuid__000000000001ecea,LM_AST* uuid__000000000001eceb){return ({LM_AST rvalue={.field_0=8};rvalue.field_8001=uuid__000000000001eceb;rvalue.field_8002=uuid__000000000001ecea;rvalue.field_8003=false_CL_U64;rvalue;});}
#line 2 "SRC/ast-location.lsts"
LM_SourceLocation _DT_location_CL_ArrowSourceLocationAST(LM_AST uuid__000000000001ecec){LM_AST uuid__000000000001eced;
LM_AST uuid__000000000001ecee;
char uuid__000000000001ecef;
LM_Token uuid__000000000001ecf0;
LM_AST uuid__000000000001ecf1;
char uuid__000000000001ecf2;
LM_Token uuid__000000000001ecf3;
LM_AST uuid__000000000001ecf4;
char uuid__000000000001ecf5;
LM_AST uuid__000000000001ecf6;
LM_AST uuid__000000000001ecf7;
LM_AST uuid__000000000001ecf8;
char uuid__000000000001ecf9;
LM_AST uuid__000000000001ecfa;
LM_AST uuid__000000000001ecfb;
LM_AST uuid__000000000001ecfc;
char uuid__000000000001ecfd;
LM_AST uuid__000000000001ecfe;
LM_AST uuid__000000000001ecff;
LM_AST uuid__000000000001ed00;
char uuid__000000000001ed01;
LM_AST uuid__000000000001ed02;
LM_Token uuid__000000000001ed03;
LM_AST uuid__000000000001ed04;
char uuid__000000000001ed05;
LM_AST uuid__000000000001ed06;
LM_AST uuid__000000000001ed07;
return ({({uuid__000000000001eced=uuid__000000000001ecec;({});})
;
(({({({({uuid__000000000001ecee=uuid__000000000001eced;({});})
;
({uuid__000000000001ecef=(0);({});})
;
});
(((uuid__000000000001ecee.field_0)==(7))?(({({uuid__000000000001ecf0=(((LM_AST)(uuid__000000000001ecee)).field_7001);({});})
;
1;
})?({uuid__000000000001ecef=(1);({});}):({})):({}));
});
(uuid__000000000001ecef==(1));
})?(uuid__000000000001ecf0.field_1):(({({({({uuid__000000000001ecf1=uuid__000000000001eced;({});})
;
({uuid__000000000001ecf2=(0);({});})
;
});
(((uuid__000000000001ecf1.field_0)==(6))?(({({uuid__000000000001ecf3=(((LM_AST)(uuid__000000000001ecf1)).field_6001);({});})
;
1;
})?({uuid__000000000001ecf2=(1);({});}):({})):({}));
});
(uuid__000000000001ecf2==(1));
})?(uuid__000000000001ecf3.field_1):(({({({({uuid__000000000001ecf4=uuid__000000000001eced;({});})
;
({uuid__000000000001ecf5=(0);({});})
;
});
(((uuid__000000000001ecf4.field_0)==(8))?(({({uuid__000000000001ecf6=(*(((LM_AST)(uuid__000000000001ecf4)).field_8001));({});})
;
1;
})?(({({uuid__000000000001ecf7=(*(((LM_AST)(uuid__000000000001ecf4)).field_8002));({});})
;
1;
})?({uuid__000000000001ecf5=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecf5==(1));
})?(_BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation((_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecf7)),(_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecf6)))):(({({({({uuid__000000000001ecf8=uuid__000000000001eced;({});})
;
({uuid__000000000001ecf9=(0);({});})
;
});
(((uuid__000000000001ecf8.field_0)==(5))?(({({uuid__000000000001ecfa=(*(((LM_AST)(uuid__000000000001ecf8)).field_5002));({});})
;
1;
})?(({({uuid__000000000001ecfb=(*(((LM_AST)(uuid__000000000001ecf8)).field_5003));({});})
;
1;
})?({uuid__000000000001ecf9=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecf9==(1));
})?(_BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation((_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecfb)),(_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecfa)))):(({({({({uuid__000000000001ecfc=uuid__000000000001eced;({});})
;
({uuid__000000000001ecfd=(0);({});})
;
});
(((uuid__000000000001ecfc.field_0)==(3))?(({({uuid__000000000001ecfe=(*(((LM_AST)(uuid__000000000001ecfc)).field_3001));({});})
;
1;
})?(({({uuid__000000000001ecff=(*(((LM_AST)(uuid__000000000001ecfc)).field_3002));({});})
;
1;
})?({uuid__000000000001ecfd=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ecfd==(1));
})?(_BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation((_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecff)),(_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ecfe)))):(({({({({uuid__000000000001ed00=uuid__000000000001eced;({});})
;
({uuid__000000000001ed01=(0);({});})
;
});
(((uuid__000000000001ed00.field_0)==(2))?(({({uuid__000000000001ed02=(*(((LM_AST)(uuid__000000000001ed00)).field_2001));({});})
;
1;
})?(({({uuid__000000000001ed03=(((LM_AST)(uuid__000000000001ed00)).field_2002);({});})
;
1;
})?({uuid__000000000001ed01=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed01==(1));
})?(_BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation((uuid__000000000001ed03.field_1),(_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ed02)))):(({({({({uuid__000000000001ed04=uuid__000000000001eced;({});})
;
({uuid__000000000001ed05=(0);({});})
;
});
(((uuid__000000000001ed04.field_0)==(1))?(({({uuid__000000000001ed06=(*(((LM_AST)(uuid__000000000001ed04)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ed07=(*(((LM_AST)(uuid__000000000001ed04)).field_1002));({});})
;
1;
})?({uuid__000000000001ed05=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed05==(1));
})?(_BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation((_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ed07)),(_DT_location_CL_ArrowSourceLocationAST(uuid__000000000001ed06)))):(1?(mk_SB_location_CL_ArrowSourceLocationNil()):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/ast-location.lsts Line: 3 Column: 4"));LM_SourceLocation rvalue;rvalue;})))))))));
});}
#line 2 "SRC/mk-location.lsts"
LM_SourceLocation mk_SB_location_CL_ArrowSourceLocationNil(){return ({LM_SourceLocation rvalue={.field_0=0};rvalue.field_1=(0);rvalue.field_2=(0);rvalue.field_3="Unknown";rvalue;});}
#line 2 "SRC/mk-token.lsts"
LM_Token mk_SB_token_CL_ArrowTokenString(char* uuid__000000000001ed08){return ({LM_Token rvalue={.field_0=0};rvalue.field_1=(mk_SB_location_CL_ArrowSourceLocationNil());rvalue.field_2=(iuid_CL_ArrowU64Nil());rvalue.field_3=uuid__000000000001ed08;rvalue;});}
#line 6 "SRC/mk-token.lsts"
LM_Token mk_SB_token_CL_ArrowTokenSmartString(LM_SmartString uuid__000000000001ed09){return ({LM_Token rvalue={.field_0=0};rvalue.field_1=(_DT_location_CL_ArrowSourceLocationSmartString(uuid__000000000001ed09));rvalue.field_2=(iuid_CL_ArrowU64Nil());rvalue.field_3=(untern_CL_ArrowStringSmartString(uuid__000000000001ed09));rvalue;});}
#line 2 "SRC/is-seq.lsts"
unsigned long _DT_is_SB_seq_CL_ArrowU64AST(LM_AST uuid__000000000001ed0a){LM_AST uuid__000000000001ed0b;
LM_AST uuid__000000000001ed0c;
return ({({uuid__000000000001ed0b=uuid__000000000001ed0a;({});})
;
(({({uuid__000000000001ed0c=uuid__000000000001ed0b;({});})
;
((uuid__000000000001ed0c.field_0)==(3));
})?true_CL_U64:(1?false_CL_U64:({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/is-seq.lsts Line: 3 Column: 4"));unsigned long rvalue;rvalue;})));
});}
#line 2 "SRC/non-zero.lsts"
unsigned long non_SB_zero_CL_ArrowU64SourceLocation(LM_SourceLocation uuid__000000000001ed0d){return ((uuid__000000000001ed0d.field_2)!=(0));}
#line 6 "SRC/non-zero.lsts"
LM_SourceLocation _BR__BR__CL_ArrowSourceLocationConsSourceLocationSourceLocation(LM_SourceLocation uuid__000000000001ed0e,LM_SourceLocation uuid__000000000001ed0f){return ((non_SB_zero_CL_ArrowU64SourceLocation(uuid__000000000001ed0e))?uuid__000000000001ed0e:uuid__000000000001ed0f);}
#line 4 "SRC/uuid.lsts"
char* uuid_CL_ArrowStringNil(){return ({({uuid_SB_counter_CL_U64=(uuid_SB_counter_CL_U64+(1));({});});
(clone_SB_rope_CL_ArrowStringS(({LM_S rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001=(to_SB_hex_CL_ArrowStringU64(uuid_SB_counter_CL_U64));rvalue;})));rvalue.field_1002=(close_CL_ArrowArray_QM_SS(({LM_S rvalue={.field_0=2};rvalue.field_2001="uuid__";rvalue;})));rvalue;})));
});}
#line 12 "SRC/uuid.lsts"
unsigned long iuid_CL_ArrowU64Nil(){return ({({uuid_SB_counter_CL_U64=(uuid_SB_counter_CL_U64+(1));({});});
uuid_SB_counter_CL_U64;
});}
#line 4 "SRC/smart-token-location.lsts"
LM_SourceLocation _DT_location_CL_ArrowSourceLocationSmartString(LM_SmartString uuid__000000000001ed12){LM_SmartString uuid__000000000001ed13;
unsigned long uuid__000000000001ed16;
unsigned long uuid__000000000001ed17;
char* uuid__000000000001ed18;
return ({({({({({({uuid__000000000001ed13=(_DT_lookup_CL_ArrowSmartStringConsSmartStringConsU64HashtableEqSmartStringU64(smart_SB_token_SB_path_SB_index_CL_HashtableEqSmartStringU64,((unsigned long)((uuid__000000000001ed12.field_4))),uuid__000000000001ed14));({});})
;
({uuid__000000000001ed16=(1);({});})
;
});
({uuid__000000000001ed17=(1);({});})
;
});
({uuid__000000000001ed18=(uuid__000000000001ed12.field_4);({});})
;
});
({while((uuid__000000000001ed18<(uuid__000000000001ed12.field_3))){((void)(({(((uuid__000000000001ed18[(0)])==(10))?({({uuid__000000000001ed16=(uuid__000000000001ed16+(1));({});});
({uuid__000000000001ed17=(1);({});});
}):({uuid__000000000001ed17=(uuid__000000000001ed17+(1));({});}));
({uuid__000000000001ed18=(uuid__000000000001ed18+(1));({});});
})));};});
});
({LM_SourceLocation rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ed17;rvalue.field_2=uuid__000000000001ed16;rvalue.field_3=(untern_CL_ArrowStringSmartString(uuid__000000000001ed13));rvalue;});
});}
#line 4 "SRC/inference-definitions.lsts"
unsigned long non_SB_zero_CL_ArrowU64TContext(LM_TContext uuid__000000000001ed19){return ((uuid__000000000001ed19.field_0)!=(({LM_TContext rvalue={2};rvalue;}).field_0));}
#line 2 "SRC/can-unify.lm"
unsigned long can_SB_unify_CL_ArrowU64ConsListTypeListType(LM_ListType uuid__000000000001ed1a,LM_ListType uuid__000000000001ed1b){unsigned long uuid__000000000001ed1c;
LM_TupleListTypeListType uuid__000000000001ed1d;
LM_TupleListTypeListType uuid__000000000001ed1e;
char uuid__000000000001ed1f;
LM_ListType uuid__000000000001ed20;
LM_ListType uuid__000000000001ed21;
LM_TupleListTypeListType uuid__000000000001ed22;
char uuid__000000000001ed23;
LM_ListType uuid__000000000001ed24;
char uuid__000000000001ed25;
LM_ListType uuid__000000000001ed26;
LM_Type uuid__000000000001ed27;
LM_ListType uuid__000000000001ed28;
char uuid__000000000001ed29;
LM_ListType uuid__000000000001ed2a;
LM_Type uuid__000000000001ed2b;
return ({({({uuid__000000000001ed1c=(0);({});})
;
({({uuid__000000000001ed1d=({LM_TupleListTypeListType rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ed1b;rvalue.field_2=uuid__000000000001ed1a;rvalue;});({});})
;
(({({({({uuid__000000000001ed1e=uuid__000000000001ed1d;({});})
;
({uuid__000000000001ed1f=(0);({});})
;
});
(((uuid__000000000001ed1e.field_0)==(0))?(({({uuid__000000000001ed20=(((LM_TupleListTypeListType)(uuid__000000000001ed1e)).field_1);({});})
;
((uuid__000000000001ed20.field_0)==(1));
})?(({({uuid__000000000001ed21=(((LM_TupleListTypeListType)(uuid__000000000001ed1e)).field_2);({});})
;
((uuid__000000000001ed21.field_0)==(1));
})?({uuid__000000000001ed1f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed1f==(1));
})?({uuid__000000000001ed1c=(1);({});}):(({({({({uuid__000000000001ed22=uuid__000000000001ed1d;({});})
;
({uuid__000000000001ed23=(0);({});})
;
});
(((uuid__000000000001ed22.field_0)==(0))?(({({({({uuid__000000000001ed24=(((LM_TupleListTypeListType)(uuid__000000000001ed22)).field_1);({});})
;
({uuid__000000000001ed25=(0);({});})
;
});
(((uuid__000000000001ed24.field_0)==(0))?(({({uuid__000000000001ed26=(*(((LM_ListType)(uuid__000000000001ed24)).field_1));({});})
;
1;
})?(({({uuid__000000000001ed27=(((LM_ListType)(uuid__000000000001ed24)).field_2);({});})
;
1;
})?({uuid__000000000001ed25=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed25==(1));
})?(({({({({uuid__000000000001ed28=(((LM_TupleListTypeListType)(uuid__000000000001ed22)).field_2);({});})
;
({uuid__000000000001ed29=(0);({});})
;
});
(((uuid__000000000001ed28.field_0)==(0))?(({({uuid__000000000001ed2a=(*(((LM_ListType)(uuid__000000000001ed28)).field_1));({});})
;
1;
})?(({({uuid__000000000001ed2b=(((LM_ListType)(uuid__000000000001ed28)).field_2);({});})
;
1;
})?({uuid__000000000001ed29=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed29==(1));
})?({uuid__000000000001ed23=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed23==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed2b,uuid__000000000001ed27))?((can_SB_unify_CL_ArrowU64ConsListTypeListType(uuid__000000000001ed2a,uuid__000000000001ed26))?({uuid__000000000001ed1c=(1);({});}):({})):({})):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/can-unify.lm Line: 4 Column: 5")))));
});
});
uuid__000000000001ed1c;
});}
#line 19 "SRC/can-unify.lm"
unsigned long can_SB_unify_CL_ArrowU64ConsTypeType(LM_Type uuid__000000000001ed2c,LM_Type uuid__000000000001ed2d){unsigned long uuid__000000000001ed2e;
LM_TupleTypeType uuid__000000000001ed2f;
LM_TupleTypeType uuid__000000000001ed42;
char uuid__000000000001ed43;
LM_Type uuid__000000000001ed44;
LM_TupleTypeType uuid__000000000001ed45;
char uuid__000000000001ed46;
LM_Type uuid__000000000001ed47;
char uuid__000000000001ed48;
char* uuid__000000000001ed49;
LM_TupleTypeType uuid__000000000001ed4a;
char uuid__000000000001ed4b;
LM_Type uuid__000000000001ed4c;
LM_Type uuid__000000000001ed4d;
char uuid__000000000001ed4e;
char* uuid__000000000001ed4f;
LM_TupleTypeType uuid__000000000001ed50;
char uuid__000000000001ed51;
LM_Type uuid__000000000001ed52;
LM_Type uuid__000000000001ed53;
char uuid__000000000001ed54;
LM_Type uuid__000000000001ed55;
LM_Type uuid__000000000001ed56;
LM_TupleTypeType uuid__000000000001ed57;
char uuid__000000000001ed58;
LM_Type uuid__000000000001ed59;
char uuid__000000000001ed5a;
LM_Type uuid__000000000001ed5b;
LM_Type uuid__000000000001ed5c;
LM_Type uuid__000000000001ed5d;
LM_TupleTypeType uuid__000000000001ed5e;
char uuid__000000000001ed5f;
LM_Type uuid__000000000001ed60;
char uuid__000000000001ed61;
LM_ListType uuid__000000000001ed62;
char* uuid__000000000001ed63;
LM_Type uuid__000000000001ed64;
char uuid__000000000001ed65;
LM_ListType uuid__000000000001ed66;
char* uuid__000000000001ed67;
LM_TupleTypeType uuid__000000000001ed68;
char uuid__000000000001ed69;
LM_Type uuid__000000000001ed6a;
char uuid__000000000001ed6b;
LM_ListType uuid__000000000001ed6c;
char uuid__000000000001ed6d;
LM_ListType uuid__000000000001ed6e;
char uuid__000000000001ed6f;
LM_ListType uuid__000000000001ed70;
LM_Type uuid__000000000001ed71;
char uuid__000000000001ed72;
char* uuid__000000000001ed73;
LM_Type uuid__000000000001ed74;
char uuid__000000000001ed75;
char* uuid__000000000001ed76;
char* uuid__000000000001ed77;
LM_Type uuid__000000000001ed78;
char uuid__000000000001ed79;
LM_ListType uuid__000000000001ed7a;
char uuid__000000000001ed7b;
LM_ListType uuid__000000000001ed7c;
char uuid__000000000001ed7d;
LM_ListType uuid__000000000001ed7e;
LM_Type uuid__000000000001ed7f;
char uuid__000000000001ed80;
char* uuid__000000000001ed81;
LM_Type uuid__000000000001ed82;
char uuid__000000000001ed83;
char* uuid__000000000001ed84;
char* uuid__000000000001ed85;
LM_TupleTypeType uuid__000000000001ed86;
char uuid__000000000001ed87;
LM_Type uuid__000000000001ed88;
char uuid__000000000001ed89;
LM_ListType uuid__000000000001ed8a;
char uuid__000000000001ed8b;
LM_ListType uuid__000000000001ed8c;
char uuid__000000000001ed8d;
LM_ListType uuid__000000000001ed8e;
LM_Type uuid__000000000001ed8f;
LM_Type uuid__000000000001ed90;
char* uuid__000000000001ed91;
LM_Type uuid__000000000001ed92;
char uuid__000000000001ed93;
LM_ListType uuid__000000000001ed94;
char uuid__000000000001ed95;
LM_ListType uuid__000000000001ed96;
char uuid__000000000001ed97;
LM_ListType uuid__000000000001ed98;
LM_Type uuid__000000000001ed99;
LM_Type uuid__000000000001ed9a;
char uuid__000000000001ed9b;
LM_ListType uuid__000000000001ed9c;
char uuid__000000000001ed9d;
LM_ListType uuid__000000000001ed9e;
LM_Type uuid__000000000001ed9f;
char* uuid__000000000001eda0;
char* uuid__000000000001eda1;
LM_TupleTypeType uuid__000000000001eda2;
char uuid__000000000001eda3;
LM_Type uuid__000000000001eda4;
LM_Type uuid__000000000001eda5;
char uuid__000000000001eda6;
LM_ListType uuid__000000000001eda7;
char uuid__000000000001eda8;
LM_ListType uuid__000000000001eda9;
char uuid__000000000001edaa;
LM_ListType uuid__000000000001edab;
LM_Type uuid__000000000001edac;
LM_Type uuid__000000000001edad;
char uuid__000000000001edae;
LM_ListType uuid__000000000001edaf;
char uuid__000000000001edb0;
LM_ListType uuid__000000000001edb1;
LM_Type uuid__000000000001edb2;
char* uuid__000000000001edb3;
char* uuid__000000000001edb4;
LM_TupleTypeType uuid__000000000001edb5;
char uuid__000000000001edb6;
LM_Type uuid__000000000001edb7;
char uuid__000000000001edb8;
LM_ListType uuid__000000000001edb9;
char uuid__000000000001edba;
LM_ListType uuid__000000000001edbb;
char uuid__000000000001edbc;
LM_ListType uuid__000000000001edbd;
LM_Type uuid__000000000001edbe;
LM_Type uuid__000000000001edbf;
char* uuid__000000000001edc0;
LM_Type uuid__000000000001edc1;
char uuid__000000000001edc2;
LM_ListType uuid__000000000001edc3;
char uuid__000000000001edc4;
LM_ListType uuid__000000000001edc5;
LM_Type uuid__000000000001edc6;
char* uuid__000000000001edc7;
LM_TupleTypeType uuid__000000000001edc8;
char uuid__000000000001edc9;
LM_Type uuid__000000000001edca;
char uuid__000000000001edcb;
LM_ListType uuid__000000000001edcc;
char* uuid__000000000001edcd;
LM_Type uuid__000000000001edce;
char uuid__000000000001edcf;
LM_ListType uuid__000000000001edd0;
char uuid__000000000001edd1;
LM_ListType uuid__000000000001edd2;
LM_Type uuid__000000000001edd3;
char* uuid__000000000001edd4;
LM_TupleTypeType uuid__000000000001edd5;
char uuid__000000000001edd6;
LM_Type uuid__000000000001edd7;
LM_Type uuid__000000000001edd8;
char uuid__000000000001edd9;
LM_ListType uuid__000000000001edda;
char uuid__000000000001eddb;
LM_ListType uuid__000000000001eddc;
LM_Type uuid__000000000001eddd;
char* uuid__000000000001edde;
LM_TupleTypeType uuid__000000000001eddf;
char uuid__000000000001ede0;
LM_Type uuid__000000000001ede1;
char uuid__000000000001ede2;
LM_ListType uuid__000000000001ede3;
char* uuid__000000000001ede4;
LM_Type uuid__000000000001ede5;
char uuid__000000000001ede6;
LM_ListType uuid__000000000001ede7;
char uuid__000000000001ede8;
LM_ListType uuid__000000000001ede9;
LM_Type uuid__000000000001edea;
char uuid__000000000001edeb;
LM_ListType uuid__000000000001edec;
char* uuid__000000000001eded;
char* uuid__000000000001edee;
LM_TupleTypeType uuid__000000000001edef;
char uuid__000000000001edf0;
LM_Type uuid__000000000001edf1;
char uuid__000000000001edf2;
LM_ListType uuid__000000000001edf3;
char* uuid__000000000001edf4;
LM_Type uuid__000000000001edf5;
char uuid__000000000001edf6;
LM_ListType uuid__000000000001edf7;
char* uuid__000000000001edf8;
return ({({({uuid__000000000001ed2e=(0);({});})
;
({({uuid__000000000001ed2f=({LM_TupleTypeType rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ed2d;rvalue.field_2=uuid__000000000001ed2c;rvalue;});({});})
;
(({({({({uuid__000000000001ed42=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed43=(0);({});})
;
});
(((uuid__000000000001ed42.field_0)==(0))?(1?(({({uuid__000000000001ed44=(((LM_TupleTypeType)(uuid__000000000001ed42)).field_2);({});})
;
((uuid__000000000001ed44.field_0)==(3));
})?({uuid__000000000001ed43=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed43==(1));
})?({uuid__000000000001ed2e=(1);({});}):(({({({({uuid__000000000001ed45=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed46=(0);({});})
;
});
(((uuid__000000000001ed45.field_0)==(0))?(1?(({({({({uuid__000000000001ed47=(((LM_TupleTypeType)(uuid__000000000001ed45)).field_2);({});})
;
({uuid__000000000001ed48=(0);({});})
;
});
(((uuid__000000000001ed47.field_0)==(1))?(1?(({({uuid__000000000001ed49=(((LM_Type)(uuid__000000000001ed47)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed49,"Any"));
})?({uuid__000000000001ed48=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed48==(1));
})?({uuid__000000000001ed46=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed46==(1));
})?({uuid__000000000001ed2e=(1);({});}):(({({({({uuid__000000000001ed4a=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed4b=(0);({});})
;
});
(((uuid__000000000001ed4a.field_0)==(0))?(({({uuid__000000000001ed4c=(((LM_TupleTypeType)(uuid__000000000001ed4a)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001ed4d=(((LM_TupleTypeType)(uuid__000000000001ed4a)).field_2);({});})
;
({uuid__000000000001ed4e=(0);({});})
;
});
(((uuid__000000000001ed4d.field_0)==(2))?(({({uuid__000000000001ed4f=(((LM_Type)(uuid__000000000001ed4d)).field_2001);({});})
;
1;
})?({uuid__000000000001ed4e=(1);({});}):({})):({}));
});
(uuid__000000000001ed4e==(1));
})?({uuid__000000000001ed4b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed4b==(1));
})?({uuid__000000000001ed2e=(1);({});}):(({({({({uuid__000000000001ed50=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed51=(0);({});})
;
});
(((uuid__000000000001ed50.field_0)==(0))?(({({uuid__000000000001ed52=(((LM_TupleTypeType)(uuid__000000000001ed50)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001ed53=(((LM_TupleTypeType)(uuid__000000000001ed50)).field_2);({});})
;
({uuid__000000000001ed54=(0);({});})
;
});
(((uuid__000000000001ed53.field_0)==(0))?(({({uuid__000000000001ed55=(*(((LM_Type)(uuid__000000000001ed53)).field_1));({});})
;
1;
})?(({({uuid__000000000001ed56=(*(((LM_Type)(uuid__000000000001ed53)).field_2));({});})
;
1;
})?({uuid__000000000001ed54=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed54==(1));
})?({uuid__000000000001ed51=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed51==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed56,uuid__000000000001ed52))?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed55,uuid__000000000001ed52))?({uuid__000000000001ed2e=(1);({});}):({})):({})):(({({({({uuid__000000000001ed57=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed58=(0);({});})
;
});
(((uuid__000000000001ed57.field_0)==(0))?(({({({({uuid__000000000001ed59=(((LM_TupleTypeType)(uuid__000000000001ed57)).field_1);({});})
;
({uuid__000000000001ed5a=(0);({});})
;
});
(((uuid__000000000001ed59.field_0)==(0))?(({({uuid__000000000001ed5b=(*(((LM_Type)(uuid__000000000001ed59)).field_1));({});})
;
1;
})?(({({uuid__000000000001ed5c=(*(((LM_Type)(uuid__000000000001ed59)).field_2));({});})
;
1;
})?({uuid__000000000001ed5a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed5a==(1));
})?(({({uuid__000000000001ed5d=(((LM_TupleTypeType)(uuid__000000000001ed57)).field_2);({});})
;
1;
})?({uuid__000000000001ed58=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed58==(1));
})?({((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed5d,uuid__000000000001ed5c))?({uuid__000000000001ed2e=(1);({});}):({}));
(uuid__000000000001ed2e?({}):({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed5d,uuid__000000000001ed5b));({});}));
}):(({({({({uuid__000000000001ed5e=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed5f=(0);({});})
;
});
(((uuid__000000000001ed5e.field_0)==(0))?(({({({({uuid__000000000001ed60=(((LM_TupleTypeType)(uuid__000000000001ed5e)).field_1);({});})
;
({uuid__000000000001ed61=(0);({});})
;
});
(((uuid__000000000001ed60.field_0)==(1))?(({({uuid__000000000001ed62=(*(((LM_Type)(uuid__000000000001ed60)).field_1001));({});})
;
((uuid__000000000001ed62.field_0)==(1));
})?(({({uuid__000000000001ed63=(((LM_Type)(uuid__000000000001ed60)).field_1002);({});})
;
1;
})?({uuid__000000000001ed61=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed61==(1));
})?(({({({({uuid__000000000001ed64=(((LM_TupleTypeType)(uuid__000000000001ed5e)).field_2);({});})
;
({uuid__000000000001ed65=(0);({});})
;
});
(((uuid__000000000001ed64.field_0)==(1))?(({({uuid__000000000001ed66=(*(((LM_Type)(uuid__000000000001ed64)).field_1001));({});})
;
((uuid__000000000001ed66.field_0)==(1));
})?(({({uuid__000000000001ed67=(((LM_Type)(uuid__000000000001ed64)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed67,"CONST"));
})?({uuid__000000000001ed65=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed65==(1));
})?({uuid__000000000001ed5f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed5f==(1));
})?({uuid__000000000001ed2e=((_DT_is_SB_digit_CL_ArrowU64String(uuid__000000000001ed63))||(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed63,"CONST")));({});}):(({({({({uuid__000000000001ed68=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed69=(0);({});})
;
});
(((uuid__000000000001ed68.field_0)==(0))?(({({({({uuid__000000000001ed6a=(((LM_TupleTypeType)(uuid__000000000001ed68)).field_1);({});})
;
({uuid__000000000001ed6b=(0);({});})
;
});
(((uuid__000000000001ed6a.field_0)==(1))?(({({({({uuid__000000000001ed6c=(*(((LM_Type)(uuid__000000000001ed6a)).field_1001));({});})
;
({uuid__000000000001ed6d=(0);({});})
;
});
(((uuid__000000000001ed6c.field_0)==(0))?(({({({({uuid__000000000001ed6e=(*(((LM_ListType)(uuid__000000000001ed6c)).field_1));({});})
;
({uuid__000000000001ed6f=(0);({});})
;
});
(((uuid__000000000001ed6e.field_0)==(0))?(({({uuid__000000000001ed70=(*(((LM_ListType)(uuid__000000000001ed6e)).field_1));({});})
;
((uuid__000000000001ed70.field_0)==(1));
})?(({({({({uuid__000000000001ed71=(((LM_ListType)(uuid__000000000001ed6e)).field_2);({});})
;
({uuid__000000000001ed72=(0);({});})
;
});
(((uuid__000000000001ed71.field_0)==(1))?(1?(({({uuid__000000000001ed73=(((LM_Type)(uuid__000000000001ed71)).field_1002);({});})
;
1;
})?({uuid__000000000001ed72=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed72==(1));
})?({uuid__000000000001ed6f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed6f==(1));
})?(({({({({uuid__000000000001ed74=(((LM_ListType)(uuid__000000000001ed6c)).field_2);({});})
;
({uuid__000000000001ed75=(0);({});})
;
});
(((uuid__000000000001ed74.field_0)==(1))?(1?(({({uuid__000000000001ed76=(((LM_Type)(uuid__000000000001ed74)).field_1002);({});})
;
1;
})?({uuid__000000000001ed75=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed75==(1));
})?({uuid__000000000001ed6d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed6d==(1));
})?(({({uuid__000000000001ed77=(((LM_Type)(uuid__000000000001ed6a)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed77,"Phi"));
})?({uuid__000000000001ed6b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed6b==(1));
})?(({({({({uuid__000000000001ed78=(((LM_TupleTypeType)(uuid__000000000001ed68)).field_2);({});})
;
({uuid__000000000001ed79=(0);({});})
;
});
(((uuid__000000000001ed78.field_0)==(1))?(({({({({uuid__000000000001ed7a=(*(((LM_Type)(uuid__000000000001ed78)).field_1001));({});})
;
({uuid__000000000001ed7b=(0);({});})
;
});
(((uuid__000000000001ed7a.field_0)==(0))?(({({({({uuid__000000000001ed7c=(*(((LM_ListType)(uuid__000000000001ed7a)).field_1));({});})
;
({uuid__000000000001ed7d=(0);({});})
;
});
(((uuid__000000000001ed7c.field_0)==(0))?(({({uuid__000000000001ed7e=(*(((LM_ListType)(uuid__000000000001ed7c)).field_1));({});})
;
((uuid__000000000001ed7e.field_0)==(1));
})?(({({({({uuid__000000000001ed7f=(((LM_ListType)(uuid__000000000001ed7c)).field_2);({});})
;
({uuid__000000000001ed80=(0);({});})
;
});
(((uuid__000000000001ed7f.field_0)==(1))?(1?(({({uuid__000000000001ed81=(((LM_Type)(uuid__000000000001ed7f)).field_1002);({});})
;
1;
})?({uuid__000000000001ed80=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed80==(1));
})?({uuid__000000000001ed7d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed7d==(1));
})?(({({({({uuid__000000000001ed82=(((LM_ListType)(uuid__000000000001ed7a)).field_2);({});})
;
({uuid__000000000001ed83=(0);({});})
;
});
(((uuid__000000000001ed82.field_0)==(1))?(1?(({({uuid__000000000001ed84=(((LM_Type)(uuid__000000000001ed82)).field_1002);({});})
;
1;
})?({uuid__000000000001ed83=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed83==(1));
})?({uuid__000000000001ed7b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed7b==(1));
})?(({({uuid__000000000001ed85=(((LM_Type)(uuid__000000000001ed78)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed85,"Phi"));
})?({uuid__000000000001ed79=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed79==(1));
})?({uuid__000000000001ed69=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed69==(1));
})?({uuid__000000000001ed2e=(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed81,uuid__000000000001ed76));({});}):(({({({({uuid__000000000001ed86=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ed87=(0);({});})
;
});
(((uuid__000000000001ed86.field_0)==(0))?(({({({({uuid__000000000001ed88=(((LM_TupleTypeType)(uuid__000000000001ed86)).field_1);({});})
;
({uuid__000000000001ed89=(0);({});})
;
});
(((uuid__000000000001ed88.field_0)==(1))?(({({({({uuid__000000000001ed8a=(*(((LM_Type)(uuid__000000000001ed88)).field_1001));({});})
;
({uuid__000000000001ed8b=(0);({});})
;
});
(((uuid__000000000001ed8a.field_0)==(0))?(({({({({uuid__000000000001ed8c=(*(((LM_ListType)(uuid__000000000001ed8a)).field_1));({});})
;
({uuid__000000000001ed8d=(0);({});})
;
});
(((uuid__000000000001ed8c.field_0)==(0))?(({({uuid__000000000001ed8e=(*(((LM_ListType)(uuid__000000000001ed8c)).field_1));({});})
;
((uuid__000000000001ed8e.field_0)==(1));
})?(({({uuid__000000000001ed8f=(((LM_ListType)(uuid__000000000001ed8c)).field_2);({});})
;
1;
})?({uuid__000000000001ed8d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed8d==(1));
})?(({({uuid__000000000001ed90=(((LM_ListType)(uuid__000000000001ed8a)).field_2);({});})
;
1;
})?({uuid__000000000001ed8b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed8b==(1));
})?(({({uuid__000000000001ed91=(((LM_Type)(uuid__000000000001ed88)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ed91,"Cons"));
})?({uuid__000000000001ed89=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed89==(1));
})?(({({({({uuid__000000000001ed92=(((LM_TupleTypeType)(uuid__000000000001ed86)).field_2);({});})
;
({uuid__000000000001ed93=(0);({});})
;
});
(((uuid__000000000001ed92.field_0)==(1))?(({({({({uuid__000000000001ed94=(*(((LM_Type)(uuid__000000000001ed92)).field_1001));({});})
;
({uuid__000000000001ed95=(0);({});})
;
});
(((uuid__000000000001ed94.field_0)==(0))?(({({({({uuid__000000000001ed96=(*(((LM_ListType)(uuid__000000000001ed94)).field_1));({});})
;
({uuid__000000000001ed97=(0);({});})
;
});
(((uuid__000000000001ed96.field_0)==(0))?(({({uuid__000000000001ed98=(*(((LM_ListType)(uuid__000000000001ed96)).field_1));({});})
;
((uuid__000000000001ed98.field_0)==(1));
})?(({({uuid__000000000001ed99=(((LM_ListType)(uuid__000000000001ed96)).field_2);({});})
;
1;
})?({uuid__000000000001ed97=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed97==(1));
})?(({({({({uuid__000000000001ed9a=(((LM_ListType)(uuid__000000000001ed94)).field_2);({});})
;
({uuid__000000000001ed9b=(0);({});})
;
});
(((uuid__000000000001ed9a.field_0)==(1))?(({({({({uuid__000000000001ed9c=(*(((LM_Type)(uuid__000000000001ed9a)).field_1001));({});})
;
({uuid__000000000001ed9d=(0);({});})
;
});
(((uuid__000000000001ed9c.field_0)==(0))?(({({uuid__000000000001ed9e=(*(((LM_ListType)(uuid__000000000001ed9c)).field_1));({});})
;
((uuid__000000000001ed9e.field_0)==(1));
})?(({({uuid__000000000001ed9f=(((LM_ListType)(uuid__000000000001ed9c)).field_2);({});})
;
1;
})?({uuid__000000000001ed9d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed9d==(1));
})?(({({uuid__000000000001eda0=(((LM_Type)(uuid__000000000001ed9a)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eda0,"..."));
})?({uuid__000000000001ed9b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed9b==(1));
})?({uuid__000000000001ed95=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed95==(1));
})?(({({uuid__000000000001eda1=(((LM_Type)(uuid__000000000001ed92)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eda1,"Cons"));
})?({uuid__000000000001ed93=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed93==(1));
})?({uuid__000000000001ed87=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ed87==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed9f,uuid__000000000001ed90))?({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed2c,uuid__000000000001ed8f));({});}):({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed99,uuid__000000000001ed2d));({});})):(({({({({uuid__000000000001eda2=uuid__000000000001ed2f;({});})
;
({uuid__000000000001eda3=(0);({});})
;
});
(((uuid__000000000001eda2.field_0)==(0))?(({({uuid__000000000001eda4=(((LM_TupleTypeType)(uuid__000000000001eda2)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001eda5=(((LM_TupleTypeType)(uuid__000000000001eda2)).field_2);({});})
;
({uuid__000000000001eda6=(0);({});})
;
});
(((uuid__000000000001eda5.field_0)==(1))?(({({({({uuid__000000000001eda7=(*(((LM_Type)(uuid__000000000001eda5)).field_1001));({});})
;
({uuid__000000000001eda8=(0);({});})
;
});
(((uuid__000000000001eda7.field_0)==(0))?(({({({({uuid__000000000001eda9=(*(((LM_ListType)(uuid__000000000001eda7)).field_1));({});})
;
({uuid__000000000001edaa=(0);({});})
;
});
(((uuid__000000000001eda9.field_0)==(0))?(({({uuid__000000000001edab=(*(((LM_ListType)(uuid__000000000001eda9)).field_1));({});})
;
((uuid__000000000001edab.field_0)==(1));
})?(({({uuid__000000000001edac=(((LM_ListType)(uuid__000000000001eda9)).field_2);({});})
;
1;
})?({uuid__000000000001edaa=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edaa==(1));
})?(({({({({uuid__000000000001edad=(((LM_ListType)(uuid__000000000001eda7)).field_2);({});})
;
({uuid__000000000001edae=(0);({});})
;
});
(((uuid__000000000001edad.field_0)==(1))?(({({({({uuid__000000000001edaf=(*(((LM_Type)(uuid__000000000001edad)).field_1001));({});})
;
({uuid__000000000001edb0=(0);({});})
;
});
(((uuid__000000000001edaf.field_0)==(0))?(({({uuid__000000000001edb1=(*(((LM_ListType)(uuid__000000000001edaf)).field_1));({});})
;
((uuid__000000000001edb1.field_0)==(1));
})?(({({uuid__000000000001edb2=(((LM_ListType)(uuid__000000000001edaf)).field_2);({});})
;
1;
})?({uuid__000000000001edb0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edb0==(1));
})?(({({uuid__000000000001edb3=(((LM_Type)(uuid__000000000001edad)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edb3,"..."));
})?({uuid__000000000001edae=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edae==(1));
})?({uuid__000000000001eda8=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eda8==(1));
})?(({({uuid__000000000001edb4=(((LM_Type)(uuid__000000000001eda5)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edb4,"Cons"));
})?({uuid__000000000001eda6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eda6==(1));
})?({uuid__000000000001eda3=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eda3==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001edb2,uuid__000000000001eda4))?({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001edac,(t1_CL_ArrowTypeString("Nil"))));({});}):({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001edac,uuid__000000000001eda4));({});})):(({({({({uuid__000000000001edb5=uuid__000000000001ed2f;({});})
;
({uuid__000000000001edb6=(0);({});})
;
});
(((uuid__000000000001edb5.field_0)==(0))?(({({({({uuid__000000000001edb7=(((LM_TupleTypeType)(uuid__000000000001edb5)).field_1);({});})
;
({uuid__000000000001edb8=(0);({});})
;
});
(((uuid__000000000001edb7.field_0)==(1))?(({({({({uuid__000000000001edb9=(*(((LM_Type)(uuid__000000000001edb7)).field_1001));({});})
;
({uuid__000000000001edba=(0);({});})
;
});
(((uuid__000000000001edb9.field_0)==(0))?(({({({({uuid__000000000001edbb=(*(((LM_ListType)(uuid__000000000001edb9)).field_1));({});})
;
({uuid__000000000001edbc=(0);({});})
;
});
(((uuid__000000000001edbb.field_0)==(0))?(({({uuid__000000000001edbd=(*(((LM_ListType)(uuid__000000000001edbb)).field_1));({});})
;
((uuid__000000000001edbd.field_0)==(1));
})?(({({uuid__000000000001edbe=(((LM_ListType)(uuid__000000000001edbb)).field_2);({});})
;
1;
})?({uuid__000000000001edbc=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edbc==(1));
})?(({({uuid__000000000001edbf=(((LM_ListType)(uuid__000000000001edb9)).field_2);({});})
;
1;
})?({uuid__000000000001edba=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edba==(1));
})?(({({uuid__000000000001edc0=(((LM_Type)(uuid__000000000001edb7)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edc0,"Cons"));
})?({uuid__000000000001edb8=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edb8==(1));
})?(({({({({uuid__000000000001edc1=(((LM_TupleTypeType)(uuid__000000000001edb5)).field_2);({});})
;
({uuid__000000000001edc2=(0);({});})
;
});
(((uuid__000000000001edc1.field_0)==(1))?(({({({({uuid__000000000001edc3=(*(((LM_Type)(uuid__000000000001edc1)).field_1001));({});})
;
({uuid__000000000001edc4=(0);({});})
;
});
(((uuid__000000000001edc3.field_0)==(0))?(({({uuid__000000000001edc5=(*(((LM_ListType)(uuid__000000000001edc3)).field_1));({});})
;
((uuid__000000000001edc5.field_0)==(1));
})?(({({uuid__000000000001edc6=(((LM_ListType)(uuid__000000000001edc3)).field_2);({});})
;
1;
})?({uuid__000000000001edc4=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edc4==(1));
})?(({({uuid__000000000001edc7=(((LM_Type)(uuid__000000000001edc1)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edc7,"..."));
})?({uuid__000000000001edc2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edc2==(1));
})?({uuid__000000000001edb6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edb6==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001edc6,uuid__000000000001edbf))?({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001ed2c,uuid__000000000001edbe));({});}):({})):(({({({({uuid__000000000001edc8=uuid__000000000001ed2f;({});})
;
({uuid__000000000001edc9=(0);({});})
;
});
(((uuid__000000000001edc8.field_0)==(0))?(({({({({uuid__000000000001edca=(((LM_TupleTypeType)(uuid__000000000001edc8)).field_1);({});})
;
({uuid__000000000001edcb=(0);({});})
;
});
(((uuid__000000000001edca.field_0)==(1))?(({({uuid__000000000001edcc=(*(((LM_Type)(uuid__000000000001edca)).field_1001));({});})
;
((uuid__000000000001edcc.field_0)==(1));
})?(({({uuid__000000000001edcd=(((LM_Type)(uuid__000000000001edca)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edcd,"Nil"));
})?({uuid__000000000001edcb=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edcb==(1));
})?(({({({({uuid__000000000001edce=(((LM_TupleTypeType)(uuid__000000000001edc8)).field_2);({});})
;
({uuid__000000000001edcf=(0);({});})
;
});
(((uuid__000000000001edce.field_0)==(1))?(({({({({uuid__000000000001edd0=(*(((LM_Type)(uuid__000000000001edce)).field_1001));({});})
;
({uuid__000000000001edd1=(0);({});})
;
});
(((uuid__000000000001edd0.field_0)==(0))?(({({uuid__000000000001edd2=(*(((LM_ListType)(uuid__000000000001edd0)).field_1));({});})
;
((uuid__000000000001edd2.field_0)==(1));
})?(({({uuid__000000000001edd3=(((LM_ListType)(uuid__000000000001edd0)).field_2);({});})
;
1;
})?({uuid__000000000001edd1=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edd1==(1));
})?(({({uuid__000000000001edd4=(((LM_Type)(uuid__000000000001edce)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edd4,"..."));
})?({uuid__000000000001edcf=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edcf==(1));
})?({uuid__000000000001edc9=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edc9==(1));
})?({uuid__000000000001ed2e=(1);({});}):(({({({({uuid__000000000001edd5=uuid__000000000001ed2f;({});})
;
({uuid__000000000001edd6=(0);({});})
;
});
(((uuid__000000000001edd5.field_0)==(0))?(({({uuid__000000000001edd7=(((LM_TupleTypeType)(uuid__000000000001edd5)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001edd8=(((LM_TupleTypeType)(uuid__000000000001edd5)).field_2);({});})
;
({uuid__000000000001edd9=(0);({});})
;
});
(((uuid__000000000001edd8.field_0)==(1))?(({({({({uuid__000000000001edda=(*(((LM_Type)(uuid__000000000001edd8)).field_1001));({});})
;
({uuid__000000000001eddb=(0);({});})
;
});
(((uuid__000000000001edda.field_0)==(0))?(({({uuid__000000000001eddc=(*(((LM_ListType)(uuid__000000000001edda)).field_1));({});})
;
((uuid__000000000001eddc.field_0)==(1));
})?(({({uuid__000000000001eddd=(((LM_ListType)(uuid__000000000001edda)).field_2);({});})
;
1;
})?({uuid__000000000001eddb=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eddb==(1));
})?(({({uuid__000000000001edde=(((LM_Type)(uuid__000000000001edd8)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edde,"..."));
})?({uuid__000000000001edd9=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edd9==(1));
})?({uuid__000000000001edd6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edd6==(1));
})?({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001eddd,uuid__000000000001edd7));({});}):(({({({({uuid__000000000001eddf=uuid__000000000001ed2f;({});})
;
({uuid__000000000001ede0=(0);({});})
;
});
(((uuid__000000000001eddf.field_0)==(0))?(({({({({uuid__000000000001ede1=(((LM_TupleTypeType)(uuid__000000000001eddf)).field_1);({});})
;
({uuid__000000000001ede2=(0);({});})
;
});
(((uuid__000000000001ede1.field_0)==(1))?(({({uuid__000000000001ede3=(*(((LM_Type)(uuid__000000000001ede1)).field_1001));({});})
;
((uuid__000000000001ede3.field_0)==(1));
})?(({({uuid__000000000001ede4=(((LM_Type)(uuid__000000000001ede1)).field_1002);({});})
;
1;
})?({uuid__000000000001ede2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ede2==(1));
})?(({({({({uuid__000000000001ede5=(((LM_TupleTypeType)(uuid__000000000001eddf)).field_2);({});})
;
({uuid__000000000001ede6=(0);({});})
;
});
(((uuid__000000000001ede5.field_0)==(1))?(({({({({uuid__000000000001ede7=(*(((LM_Type)(uuid__000000000001ede5)).field_1001));({});})
;
({uuid__000000000001ede8=(0);({});})
;
});
(((uuid__000000000001ede7.field_0)==(0))?(({({uuid__000000000001ede9=(*(((LM_ListType)(uuid__000000000001ede7)).field_1));({});})
;
((uuid__000000000001ede9.field_0)==(1));
})?(({({({({uuid__000000000001edea=(((LM_ListType)(uuid__000000000001ede7)).field_2);({});})
;
({uuid__000000000001edeb=(0);({});})
;
});
(((uuid__000000000001edea.field_0)==(1))?(({({uuid__000000000001edec=(*(((LM_Type)(uuid__000000000001edea)).field_1001));({});})
;
((uuid__000000000001edec.field_0)==(1));
})?(({({uuid__000000000001eded=(((LM_Type)(uuid__000000000001edea)).field_1002);({});})
;
1;
})?({uuid__000000000001edeb=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edeb==(1));
})?({uuid__000000000001ede8=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ede8==(1));
})?(({({uuid__000000000001edee=(((LM_Type)(uuid__000000000001ede5)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edee,"GT"));
})?({uuid__000000000001ede6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ede6==(1));
})?({uuid__000000000001ede0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ede0==(1));
})?(((to_SB_i64_CL_ArrowI64String(uuid__000000000001ede4))>(to_SB_i64_CL_ArrowI64String(uuid__000000000001eded)))?({uuid__000000000001ed2e=(1);({});}):({})):(({({({({uuid__000000000001edef=uuid__000000000001ed2f;({});})
;
({uuid__000000000001edf0=(0);({});})
;
});
(((uuid__000000000001edef.field_0)==(0))?(({({({({uuid__000000000001edf1=(((LM_TupleTypeType)(uuid__000000000001edef)).field_1);({});})
;
({uuid__000000000001edf2=(0);({});})
;
});
(((uuid__000000000001edf1.field_0)==(1))?(({({uuid__000000000001edf3=(*(((LM_Type)(uuid__000000000001edf1)).field_1001));({});})
;
1;
})?(({({uuid__000000000001edf4=(((LM_Type)(uuid__000000000001edf1)).field_1002);({});})
;
1;
})?({uuid__000000000001edf2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edf2==(1));
})?(({({({({uuid__000000000001edf5=(((LM_TupleTypeType)(uuid__000000000001edef)).field_2);({});})
;
({uuid__000000000001edf6=(0);({});})
;
});
(((uuid__000000000001edf5.field_0)==(1))?(({({uuid__000000000001edf7=(*(((LM_Type)(uuid__000000000001edf5)).field_1001));({});})
;
1;
})?(({({uuid__000000000001edf8=(((LM_Type)(uuid__000000000001edf5)).field_1002);({});})
;
1;
})?({uuid__000000000001edf6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edf6==(1));
})?({uuid__000000000001edf0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001edf0==(1));
})?((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001edf8,uuid__000000000001edf4))?({uuid__000000000001ed2e=(can_SB_unify_CL_ArrowU64ConsListTypeListType(uuid__000000000001edf7,uuid__000000000001edf3));({});}):({})):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/can-unify.lm Line: 21 Column: 5")))))))))))))))));
});
});
uuid__000000000001ed2e;
});}
#line 2 "SRC/unify.lm"
LM_TContext unify_CL_ArrowTContextConsTypeType(LM_Type uuid__000000000001edf9,LM_Type uuid__000000000001edfa){LM_TContext uuid__000000000001edfb;
return ({({({uuid__000000000001edfb=({LM_TContext rvalue={2};rvalue;});({});})
;
((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001edf9,uuid__000000000001edfa))?({uuid__000000000001edfb=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001edf9,uuid__000000000001edfa));({});}):({}));
});
uuid__000000000001edfb;
});}
#line 10 "SRC/unify.lm"
LM_TContext unify_SB_inner_CL_ArrowTContextConsTypeType(LM_Type uuid__000000000001edfc,LM_Type uuid__000000000001edfd){LM_TContext uuid__000000000001edfe;
LM_TupleTypeType uuid__000000000001edff;
LM_TupleTypeType uuid__000000000001ee12;
char uuid__000000000001ee13;
LM_Type uuid__000000000001ee14;
LM_TupleTypeType uuid__000000000001ee15;
char uuid__000000000001ee16;
LM_Type uuid__000000000001ee17;
char uuid__000000000001ee18;
LM_ListType uuid__000000000001ee19;
char* uuid__000000000001ee1a;
LM_Type uuid__000000000001ee1b;
char uuid__000000000001ee1c;
char* uuid__000000000001ee1d;
LM_TupleTypeType uuid__000000000001ee1e;
char uuid__000000000001ee1f;
LM_Type uuid__000000000001ee20;
char uuid__000000000001ee21;
char* uuid__000000000001ee22;
LM_TupleTypeType uuid__000000000001ee23;
char uuid__000000000001ee24;
LM_Type uuid__000000000001ee25;
LM_Type uuid__000000000001ee26;
char uuid__000000000001ee27;
char* uuid__000000000001ee28;
LM_TupleTypeType uuid__000000000001ee29;
char uuid__000000000001ee2a;
LM_Type uuid__000000000001ee2b;
LM_Type uuid__000000000001ee2c;
char uuid__000000000001ee2d;
LM_Type uuid__000000000001ee2e;
LM_Type uuid__000000000001ee2f;
LM_TupleTContextTContext uuid__000000000001ee30;
LM_TupleTContextTContext uuid__000000000001ee43;
char uuid__000000000001ee44;
LM_TContext uuid__000000000001ee45;
LM_TupleTContextTContext uuid__000000000001ee46;
char uuid__000000000001ee47;
LM_TContext uuid__000000000001ee48;
LM_TupleTContextTContext uuid__000000000001ee49;
char uuid__000000000001ee4a;
LM_TContext uuid__000000000001ee4b;
LM_TContext uuid__000000000001ee4c;
LM_TupleTypeType uuid__000000000001ee4d;
char uuid__000000000001ee4e;
LM_Type uuid__000000000001ee4f;
char uuid__000000000001ee50;
LM_Type uuid__000000000001ee51;
LM_Type uuid__000000000001ee52;
LM_Type uuid__000000000001ee53;
LM_TupleTContextTContext uuid__000000000001ee54;
LM_TupleTContextTContext uuid__000000000001ee67;
char uuid__000000000001ee68;
LM_TContext uuid__000000000001ee69;
LM_TContext uuid__000000000001ee6a;
LM_TupleTContextTContext uuid__000000000001ee6b;
char uuid__000000000001ee6c;
LM_TContext uuid__000000000001ee6d;
LM_TContext uuid__000000000001ee6e;
LM_TupleTContextTContext uuid__000000000001ee6f;
char uuid__000000000001ee70;
LM_TContext uuid__000000000001ee71;
LM_TContext uuid__000000000001ee72;
LM_TupleTContextTContext uuid__000000000001ee73;
char uuid__000000000001ee74;
LM_TContext uuid__000000000001ee75;
LM_TContext uuid__000000000001ee76;
LM_TupleTypeType uuid__000000000001ee77;
char uuid__000000000001ee78;
LM_Type uuid__000000000001ee79;
char uuid__000000000001ee7a;
LM_ListType uuid__000000000001ee7b;
char* uuid__000000000001ee7c;
LM_Type uuid__000000000001ee7d;
char uuid__000000000001ee7e;
LM_ListType uuid__000000000001ee7f;
char* uuid__000000000001ee80;
LM_TupleTypeType uuid__000000000001ee81;
char uuid__000000000001ee82;
LM_Type uuid__000000000001ee83;
char uuid__000000000001ee84;
LM_ListType uuid__000000000001ee85;
char uuid__000000000001ee86;
LM_ListType uuid__000000000001ee87;
char uuid__000000000001ee88;
LM_ListType uuid__000000000001ee89;
LM_Type uuid__000000000001ee8a;
char uuid__000000000001ee8b;
char* uuid__000000000001ee8c;
LM_Type uuid__000000000001ee8d;
char uuid__000000000001ee8e;
char* uuid__000000000001ee8f;
char* uuid__000000000001ee90;
LM_Type uuid__000000000001ee91;
char uuid__000000000001ee92;
LM_ListType uuid__000000000001ee93;
char uuid__000000000001ee94;
LM_ListType uuid__000000000001ee95;
char uuid__000000000001ee96;
LM_ListType uuid__000000000001ee97;
LM_Type uuid__000000000001ee98;
char uuid__000000000001ee99;
char* uuid__000000000001ee9a;
LM_Type uuid__000000000001ee9b;
char* uuid__000000000001ee9c;
LM_TupleTypeType uuid__000000000001ee9d;
char uuid__000000000001ee9e;
LM_Type uuid__000000000001ee9f;
char uuid__000000000001eea0;
LM_ListType uuid__000000000001eea1;
char uuid__000000000001eea2;
LM_ListType uuid__000000000001eea3;
char uuid__000000000001eea4;
LM_ListType uuid__000000000001eea5;
LM_Type uuid__000000000001eea6;
LM_Type uuid__000000000001eea7;
char* uuid__000000000001eea8;
LM_Type uuid__000000000001eea9;
char uuid__000000000001eeaa;
LM_ListType uuid__000000000001eeab;
char uuid__000000000001eeac;
LM_ListType uuid__000000000001eead;
char uuid__000000000001eeae;
LM_ListType uuid__000000000001eeaf;
LM_Type uuid__000000000001eeb0;
LM_Type uuid__000000000001eeb1;
char uuid__000000000001eeb2;
LM_ListType uuid__000000000001eeb3;
char uuid__000000000001eeb4;
LM_ListType uuid__000000000001eeb5;
LM_Type uuid__000000000001eeb6;
char* uuid__000000000001eeb7;
char* uuid__000000000001eeb8;
LM_TupleTypeType uuid__000000000001eeb9;
char uuid__000000000001eeba;
LM_Type uuid__000000000001eebb;
LM_Type uuid__000000000001eebc;
char uuid__000000000001eebd;
LM_ListType uuid__000000000001eebe;
char uuid__000000000001eebf;
LM_ListType uuid__000000000001eec0;
char uuid__000000000001eec1;
LM_ListType uuid__000000000001eec2;
LM_Type uuid__000000000001eec3;
LM_Type uuid__000000000001eec4;
char uuid__000000000001eec5;
LM_ListType uuid__000000000001eec6;
char uuid__000000000001eec7;
LM_ListType uuid__000000000001eec8;
LM_Type uuid__000000000001eec9;
char* uuid__000000000001eeca;
char* uuid__000000000001eecb;
LM_TupleTypeType uuid__000000000001eecc;
char uuid__000000000001eecd;
LM_Type uuid__000000000001eece;
char uuid__000000000001eecf;
LM_ListType uuid__000000000001eed0;
char uuid__000000000001eed1;
LM_ListType uuid__000000000001eed2;
char uuid__000000000001eed3;
LM_ListType uuid__000000000001eed4;
LM_Type uuid__000000000001eed5;
LM_Type uuid__000000000001eed6;
char* uuid__000000000001eed7;
LM_Type uuid__000000000001eed8;
char uuid__000000000001eed9;
LM_ListType uuid__000000000001eeda;
char uuid__000000000001eedb;
LM_ListType uuid__000000000001eedc;
LM_Type uuid__000000000001eedd;
char* uuid__000000000001eede;
LM_TupleTypeType uuid__000000000001eedf;
char uuid__000000000001eee0;
LM_Type uuid__000000000001eee1;
char uuid__000000000001eee2;
LM_ListType uuid__000000000001eee3;
char* uuid__000000000001eee4;
LM_Type uuid__000000000001eee5;
char uuid__000000000001eee6;
LM_ListType uuid__000000000001eee7;
char uuid__000000000001eee8;
LM_ListType uuid__000000000001eee9;
LM_Type uuid__000000000001eeea;
char* uuid__000000000001eeeb;
LM_TupleTypeType uuid__000000000001eeec;
char uuid__000000000001eeed;
LM_Type uuid__000000000001eeee;
LM_Type uuid__000000000001eeef;
char uuid__000000000001eef0;
LM_ListType uuid__000000000001eef1;
char uuid__000000000001eef2;
LM_ListType uuid__000000000001eef3;
LM_Type uuid__000000000001eef4;
char* uuid__000000000001eef5;
LM_TupleTypeType uuid__000000000001eef6;
char uuid__000000000001eef7;
LM_Type uuid__000000000001eef8;
char uuid__000000000001eef9;
LM_ListType uuid__000000000001eefa;
char* uuid__000000000001eefb;
LM_Type uuid__000000000001eefc;
char uuid__000000000001eefd;
LM_ListType uuid__000000000001eefe;
char uuid__000000000001eeff;
LM_ListType uuid__000000000001ef00;
LM_Type uuid__000000000001ef01;
char uuid__000000000001ef02;
LM_ListType uuid__000000000001ef03;
char* uuid__000000000001ef04;
char* uuid__000000000001ef05;
LM_TupleTypeType uuid__000000000001ef06;
char uuid__000000000001ef07;
LM_Type uuid__000000000001ef08;
char uuid__000000000001ef09;
LM_ListType uuid__000000000001ef0a;
char* uuid__000000000001ef0b;
LM_Type uuid__000000000001ef0c;
char uuid__000000000001ef0d;
LM_ListType uuid__000000000001ef0e;
char* uuid__000000000001ef0f;
return ({({({uuid__000000000001edfe=({LM_TContext rvalue={2};rvalue;});({});})
;
({({uuid__000000000001edff=({LM_TupleTypeType rvalue={.field_0=0};rvalue.field_1=uuid__000000000001edfd;rvalue.field_2=uuid__000000000001edfc;rvalue;});({});})
;
(({({({({uuid__000000000001ee12=uuid__000000000001edff;({});})
;
({uuid__000000000001ee13=(0);({});})
;
});
(((uuid__000000000001ee12.field_0)==(0))?(1?(({({uuid__000000000001ee14=(((LM_TupleTypeType)(uuid__000000000001ee12)).field_2);({});})
;
((uuid__000000000001ee14.field_0)==(3));
})?({uuid__000000000001ee13=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee13==(1));
})?({uuid__000000000001edfe=({LM_TContext rvalue={1};rvalue;});({});}):(({({({({uuid__000000000001ee15=uuid__000000000001edff;({});})
;
({uuid__000000000001ee16=(0);({});})
;
});
(((uuid__000000000001ee15.field_0)==(0))?(({({({({uuid__000000000001ee17=(((LM_TupleTypeType)(uuid__000000000001ee15)).field_1);({});})
;
({uuid__000000000001ee18=(0);({});})
;
});
(((uuid__000000000001ee17.field_0)==(1))?(({({uuid__000000000001ee19=(*(((LM_Type)(uuid__000000000001ee17)).field_1001));({});})
;
((uuid__000000000001ee19.field_0)==(1));
})?(({({uuid__000000000001ee1a=(((LM_Type)(uuid__000000000001ee17)).field_1002);({});})
;
1;
})?({uuid__000000000001ee18=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee18==(1));
})?(({({({({uuid__000000000001ee1b=(((LM_TupleTypeType)(uuid__000000000001ee15)).field_2);({});})
;
({uuid__000000000001ee1c=(0);({});})
;
});
(((uuid__000000000001ee1b.field_0)==(2))?(({({uuid__000000000001ee1d=(((LM_Type)(uuid__000000000001ee1b)).field_2001);({});})
;
1;
})?({uuid__000000000001ee1c=(1);({});}):({})):({}));
});
(uuid__000000000001ee1c==(1));
})?({uuid__000000000001ee16=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee16==(1));
})?({uuid__000000000001edfe=({LM_TContext rvalue={.field_0=0};rvalue.field_1=({LM_AST rvalue={.field_0=6};rvalue.field_6001=(mk_SB_token_CL_ArrowTokenString(uuid__000000000001ee1a));rvalue.field_6002=uuid__000000000001ee1a;rvalue;});rvalue.field_2=uuid__000000000001edfd;rvalue.field_3=uuid__000000000001ee1d;rvalue.field_4=(close_CL_ArrowArray_QM_TContextTContext(({LM_TContext rvalue={2};rvalue;})));rvalue;});({});}):(({({({({uuid__000000000001ee1e=uuid__000000000001edff;({});})
;
({uuid__000000000001ee1f=(0);({});})
;
});
(((uuid__000000000001ee1e.field_0)==(0))?(1?(({({({({uuid__000000000001ee20=(((LM_TupleTypeType)(uuid__000000000001ee1e)).field_2);({});})
;
({uuid__000000000001ee21=(0);({});})
;
});
(((uuid__000000000001ee20.field_0)==(1))?(1?(({({uuid__000000000001ee22=(((LM_Type)(uuid__000000000001ee20)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee22,"Any"));
})?({uuid__000000000001ee21=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee21==(1));
})?({uuid__000000000001ee1f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee1f==(1));
})?({uuid__000000000001edfe=({LM_TContext rvalue={1};rvalue;});({});}):(({({({({uuid__000000000001ee23=uuid__000000000001edff;({});})
;
({uuid__000000000001ee24=(0);({});})
;
});
(((uuid__000000000001ee23.field_0)==(0))?(({({uuid__000000000001ee25=(((LM_TupleTypeType)(uuid__000000000001ee23)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001ee26=(((LM_TupleTypeType)(uuid__000000000001ee23)).field_2);({});})
;
({uuid__000000000001ee27=(0);({});})
;
});
(((uuid__000000000001ee26.field_0)==(2))?(({({uuid__000000000001ee28=(((LM_Type)(uuid__000000000001ee26)).field_2001);({});})
;
1;
})?({uuid__000000000001ee27=(1);({});}):({})):({}));
});
(uuid__000000000001ee27==(1));
})?({uuid__000000000001ee24=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee24==(1));
})?({uuid__000000000001edfe=({LM_TContext rvalue={.field_0=0};rvalue.field_1=({LM_AST rvalue={10};rvalue;});rvalue.field_2=uuid__000000000001edfd;rvalue.field_3=uuid__000000000001ee28;rvalue.field_4=(close_CL_ArrowArray_QM_TContextTContext(({LM_TContext rvalue={2};rvalue;})));rvalue;});({});}):(({({({({uuid__000000000001ee29=uuid__000000000001edff;({});})
;
({uuid__000000000001ee2a=(0);({});})
;
});
(((uuid__000000000001ee29.field_0)==(0))?(({({uuid__000000000001ee2b=(((LM_TupleTypeType)(uuid__000000000001ee29)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001ee2c=(((LM_TupleTypeType)(uuid__000000000001ee29)).field_2);({});})
;
({uuid__000000000001ee2d=(0);({});})
;
});
(((uuid__000000000001ee2c.field_0)==(0))?(({({uuid__000000000001ee2e=(*(((LM_Type)(uuid__000000000001ee2c)).field_1));({});})
;
1;
})?(({({uuid__000000000001ee2f=(*(((LM_Type)(uuid__000000000001ee2c)).field_2));({});})
;
1;
})?({uuid__000000000001ee2d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee2d==(1));
})?({uuid__000000000001ee2a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee2a==(1));
})?({({uuid__000000000001ee30=({LM_TupleTContextTContext rvalue={.field_0=0};rvalue.field_1=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001ee2e,uuid__000000000001ee2b));rvalue.field_2=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001ee2f,uuid__000000000001ee2b));rvalue;});({});})
;
(({({({({uuid__000000000001ee43=uuid__000000000001ee30;({});})
;
({uuid__000000000001ee44=(0);({});})
;
});
(((uuid__000000000001ee43.field_0)==(0))?(1?(({({uuid__000000000001ee45=(((LM_TupleTContextTContext)(uuid__000000000001ee43)).field_2);({});})
;
((uuid__000000000001ee45.field_0)==(2));
})?({uuid__000000000001ee44=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee44==(1));
})?({}):(({({({({uuid__000000000001ee46=uuid__000000000001ee30;({});})
;
({uuid__000000000001ee47=(0);({});})
;
});
(((uuid__000000000001ee46.field_0)==(0))?(({({uuid__000000000001ee48=(((LM_TupleTContextTContext)(uuid__000000000001ee46)).field_1);({});})
;
((uuid__000000000001ee48.field_0)==(2));
})?(1?({uuid__000000000001ee47=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee47==(1));
})?({}):(({({({({uuid__000000000001ee49=uuid__000000000001ee30;({});})
;
({uuid__000000000001ee4a=(0);({});})
;
});
(((uuid__000000000001ee49.field_0)==(0))?(({({uuid__000000000001ee4b=(((LM_TupleTContextTContext)(uuid__000000000001ee49)).field_1);({});})
;
1;
})?(({({uuid__000000000001ee4c=(((LM_TupleTContextTContext)(uuid__000000000001ee49)).field_2);({});})
;
1;
})?({uuid__000000000001ee4a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee4a==(1));
})?({uuid__000000000001edfe=(union_CL_ArrowTContextConsTContextTContext(uuid__000000000001ee4c,uuid__000000000001ee4b));({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/unify.lm Line: 37 Column: 11")))));
}):(({({({({uuid__000000000001ee4d=uuid__000000000001edff;({});})
;
({uuid__000000000001ee4e=(0);({});})
;
});
(((uuid__000000000001ee4d.field_0)==(0))?(({({({({uuid__000000000001ee4f=(((LM_TupleTypeType)(uuid__000000000001ee4d)).field_1);({});})
;
({uuid__000000000001ee50=(0);({});})
;
});
(((uuid__000000000001ee4f.field_0)==(0))?(({({uuid__000000000001ee51=(*(((LM_Type)(uuid__000000000001ee4f)).field_1));({});})
;
1;
})?(({({uuid__000000000001ee52=(*(((LM_Type)(uuid__000000000001ee4f)).field_2));({});})
;
1;
})?({uuid__000000000001ee50=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee50==(1));
})?(({({uuid__000000000001ee53=(((LM_TupleTypeType)(uuid__000000000001ee4d)).field_2);({});})
;
1;
})?({uuid__000000000001ee4e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee4e==(1));
})?({({uuid__000000000001ee54=({LM_TupleTContextTContext rvalue={.field_0=0};rvalue.field_1=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001ee53,uuid__000000000001ee51));rvalue.field_2=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001ee53,uuid__000000000001ee52));rvalue;});({});})
;
(({({({({uuid__000000000001ee67=uuid__000000000001ee54;({});})
;
({uuid__000000000001ee68=(0);({});})
;
});
(((uuid__000000000001ee67.field_0)==(0))?(({({uuid__000000000001ee69=(((LM_TupleTContextTContext)(uuid__000000000001ee67)).field_1);({});})
;
((uuid__000000000001ee69.field_0)==(2));
})?(({({uuid__000000000001ee6a=(((LM_TupleTContextTContext)(uuid__000000000001ee67)).field_2);({});})
;
((uuid__000000000001ee6a.field_0)==(2));
})?({uuid__000000000001ee68=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee68==(1));
})?({}):(({({({({uuid__000000000001ee6b=uuid__000000000001ee54;({});})
;
({uuid__000000000001ee6c=(0);({});})
;
});
(((uuid__000000000001ee6b.field_0)==(0))?(({({uuid__000000000001ee6d=(((LM_TupleTContextTContext)(uuid__000000000001ee6b)).field_1);({});})
;
((uuid__000000000001ee6d.field_0)==(2));
})?(({({uuid__000000000001ee6e=(((LM_TupleTContextTContext)(uuid__000000000001ee6b)).field_2);({});})
;
1;
})?({uuid__000000000001ee6c=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee6c==(1));
})?({uuid__000000000001edfe=uuid__000000000001ee6e;({});}):(({({({({uuid__000000000001ee6f=uuid__000000000001ee54;({});})
;
({uuid__000000000001ee70=(0);({});})
;
});
(((uuid__000000000001ee6f.field_0)==(0))?(({({uuid__000000000001ee71=(((LM_TupleTContextTContext)(uuid__000000000001ee6f)).field_1);({});})
;
1;
})?(({({uuid__000000000001ee72=(((LM_TupleTContextTContext)(uuid__000000000001ee6f)).field_2);({});})
;
((uuid__000000000001ee72.field_0)==(2));
})?({uuid__000000000001ee70=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee70==(1));
})?({uuid__000000000001edfe=uuid__000000000001ee71;({});}):(({({({({uuid__000000000001ee73=uuid__000000000001ee54;({});})
;
({uuid__000000000001ee74=(0);({});})
;
});
(((uuid__000000000001ee73.field_0)==(0))?(({({uuid__000000000001ee75=(((LM_TupleTContextTContext)(uuid__000000000001ee73)).field_1);({});})
;
1;
})?(({({uuid__000000000001ee76=(((LM_TupleTContextTContext)(uuid__000000000001ee73)).field_2);({});})
;
1;
})?({uuid__000000000001ee74=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee74==(1));
})?({uuid__000000000001edfe=(union_CL_ArrowTContextConsTContextTContext(uuid__000000000001ee76,uuid__000000000001ee75));({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/unify.lm Line: 47 Column: 11"))))));
}):(({({({({uuid__000000000001ee77=uuid__000000000001edff;({});})
;
({uuid__000000000001ee78=(0);({});})
;
});
(((uuid__000000000001ee77.field_0)==(0))?(({({({({uuid__000000000001ee79=(((LM_TupleTypeType)(uuid__000000000001ee77)).field_1);({});})
;
({uuid__000000000001ee7a=(0);({});})
;
});
(((uuid__000000000001ee79.field_0)==(1))?(({({uuid__000000000001ee7b=(*(((LM_Type)(uuid__000000000001ee79)).field_1001));({});})
;
((uuid__000000000001ee7b.field_0)==(1));
})?(({({uuid__000000000001ee7c=(((LM_Type)(uuid__000000000001ee79)).field_1002);({});})
;
1;
})?({uuid__000000000001ee7a=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee7a==(1));
})?(({({({({uuid__000000000001ee7d=(((LM_TupleTypeType)(uuid__000000000001ee77)).field_2);({});})
;
({uuid__000000000001ee7e=(0);({});})
;
});
(((uuid__000000000001ee7d.field_0)==(1))?(({({uuid__000000000001ee7f=(*(((LM_Type)(uuid__000000000001ee7d)).field_1001));({});})
;
((uuid__000000000001ee7f.field_0)==(1));
})?(({({uuid__000000000001ee80=(((LM_Type)(uuid__000000000001ee7d)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee80,"CONST"));
})?({uuid__000000000001ee7e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee7e==(1));
})?({uuid__000000000001ee78=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee78==(1));
})?(((_DT_is_SB_digit_CL_ArrowU64String(uuid__000000000001ee7c))||(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee7c,"CONST")))?({uuid__000000000001edfe=({LM_TContext rvalue={1};rvalue;});({});}):({})):(({({({({uuid__000000000001ee81=uuid__000000000001edff;({});})
;
({uuid__000000000001ee82=(0);({});})
;
});
(((uuid__000000000001ee81.field_0)==(0))?(({({({({uuid__000000000001ee83=(((LM_TupleTypeType)(uuid__000000000001ee81)).field_1);({});})
;
({uuid__000000000001ee84=(0);({});})
;
});
(((uuid__000000000001ee83.field_0)==(1))?(({({({({uuid__000000000001ee85=(*(((LM_Type)(uuid__000000000001ee83)).field_1001));({});})
;
({uuid__000000000001ee86=(0);({});})
;
});
(((uuid__000000000001ee85.field_0)==(0))?(({({({({uuid__000000000001ee87=(*(((LM_ListType)(uuid__000000000001ee85)).field_1));({});})
;
({uuid__000000000001ee88=(0);({});})
;
});
(((uuid__000000000001ee87.field_0)==(0))?(({({uuid__000000000001ee89=(*(((LM_ListType)(uuid__000000000001ee87)).field_1));({});})
;
((uuid__000000000001ee89.field_0)==(1));
})?(({({({({uuid__000000000001ee8a=(((LM_ListType)(uuid__000000000001ee87)).field_2);({});})
;
({uuid__000000000001ee8b=(0);({});})
;
});
(((uuid__000000000001ee8a.field_0)==(1))?(1?(({({uuid__000000000001ee8c=(((LM_Type)(uuid__000000000001ee8a)).field_1002);({});})
;
1;
})?({uuid__000000000001ee8b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee8b==(1));
})?({uuid__000000000001ee88=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee88==(1));
})?(({({({({uuid__000000000001ee8d=(((LM_ListType)(uuid__000000000001ee85)).field_2);({});})
;
({uuid__000000000001ee8e=(0);({});})
;
});
(((uuid__000000000001ee8d.field_0)==(1))?(1?(({({uuid__000000000001ee8f=(((LM_Type)(uuid__000000000001ee8d)).field_1002);({});})
;
1;
})?({uuid__000000000001ee8e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee8e==(1));
})?({uuid__000000000001ee86=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee86==(1));
})?(({({uuid__000000000001ee90=(((LM_Type)(uuid__000000000001ee83)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee90,"Phi"));
})?({uuid__000000000001ee84=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee84==(1));
})?(({({({({uuid__000000000001ee91=(((LM_TupleTypeType)(uuid__000000000001ee81)).field_2);({});})
;
({uuid__000000000001ee92=(0);({});})
;
});
(((uuid__000000000001ee91.field_0)==(1))?(({({({({uuid__000000000001ee93=(*(((LM_Type)(uuid__000000000001ee91)).field_1001));({});})
;
({uuid__000000000001ee94=(0);({});})
;
});
(((uuid__000000000001ee93.field_0)==(0))?(({({({({uuid__000000000001ee95=(*(((LM_ListType)(uuid__000000000001ee93)).field_1));({});})
;
({uuid__000000000001ee96=(0);({});})
;
});
(((uuid__000000000001ee95.field_0)==(0))?(({({uuid__000000000001ee97=(*(((LM_ListType)(uuid__000000000001ee95)).field_1));({});})
;
((uuid__000000000001ee97.field_0)==(1));
})?(({({({({uuid__000000000001ee98=(((LM_ListType)(uuid__000000000001ee95)).field_2);({});})
;
({uuid__000000000001ee99=(0);({});})
;
});
(((uuid__000000000001ee98.field_0)==(1))?(1?(({({uuid__000000000001ee9a=(((LM_Type)(uuid__000000000001ee98)).field_1002);({});})
;
1;
})?({uuid__000000000001ee99=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee99==(1));
})?({uuid__000000000001ee96=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee96==(1));
})?(({({uuid__000000000001ee9b=(((LM_ListType)(uuid__000000000001ee93)).field_2);({});})
;
1;
})?({uuid__000000000001ee94=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee94==(1));
})?(({({uuid__000000000001ee9c=(((LM_Type)(uuid__000000000001ee91)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee9c,"Phi"));
})?({uuid__000000000001ee92=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee92==(1));
})?({uuid__000000000001ee82=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee82==(1));
})?((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ee9a,uuid__000000000001ee8f))?({uuid__000000000001edfe=({LM_TContext rvalue={.field_0=0};rvalue.field_1=({LM_AST rvalue={.field_0=7};rvalue.field_7001=(mk_SB_token_CL_ArrowTokenString(uuid__000000000001ee8c));rvalue.field_7002=uuid__000000000001ee8c;rvalue;});rvalue.field_2=uuid__000000000001ee9b;rvalue.field_3="Phi::Transition";rvalue.field_4=(close_CL_ArrowArray_QM_TContextTContext(({LM_TContext rvalue={2};rvalue;})));rvalue;});({});}):({LM_TContext rvalue={2};rvalue;})):(({({({({uuid__000000000001ee9d=uuid__000000000001edff;({});})
;
({uuid__000000000001ee9e=(0);({});})
;
});
(((uuid__000000000001ee9d.field_0)==(0))?(({({({({uuid__000000000001ee9f=(((LM_TupleTypeType)(uuid__000000000001ee9d)).field_1);({});})
;
({uuid__000000000001eea0=(0);({});})
;
});
(((uuid__000000000001ee9f.field_0)==(1))?(({({({({uuid__000000000001eea1=(*(((LM_Type)(uuid__000000000001ee9f)).field_1001));({});})
;
({uuid__000000000001eea2=(0);({});})
;
});
(((uuid__000000000001eea1.field_0)==(0))?(({({({({uuid__000000000001eea3=(*(((LM_ListType)(uuid__000000000001eea1)).field_1));({});})
;
({uuid__000000000001eea4=(0);({});})
;
});
(((uuid__000000000001eea3.field_0)==(0))?(({({uuid__000000000001eea5=(*(((LM_ListType)(uuid__000000000001eea3)).field_1));({});})
;
((uuid__000000000001eea5.field_0)==(1));
})?(({({uuid__000000000001eea6=(((LM_ListType)(uuid__000000000001eea3)).field_2);({});})
;
1;
})?({uuid__000000000001eea4=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eea4==(1));
})?(({({uuid__000000000001eea7=(((LM_ListType)(uuid__000000000001eea1)).field_2);({});})
;
1;
})?({uuid__000000000001eea2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eea2==(1));
})?(({({uuid__000000000001eea8=(((LM_Type)(uuid__000000000001ee9f)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eea8,"Cons"));
})?({uuid__000000000001eea0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eea0==(1));
})?(({({({({uuid__000000000001eea9=(((LM_TupleTypeType)(uuid__000000000001ee9d)).field_2);({});})
;
({uuid__000000000001eeaa=(0);({});})
;
});
(((uuid__000000000001eea9.field_0)==(1))?(({({({({uuid__000000000001eeab=(*(((LM_Type)(uuid__000000000001eea9)).field_1001));({});})
;
({uuid__000000000001eeac=(0);({});})
;
});
(((uuid__000000000001eeab.field_0)==(0))?(({({({({uuid__000000000001eead=(*(((LM_ListType)(uuid__000000000001eeab)).field_1));({});})
;
({uuid__000000000001eeae=(0);({});})
;
});
(((uuid__000000000001eead.field_0)==(0))?(({({uuid__000000000001eeaf=(*(((LM_ListType)(uuid__000000000001eead)).field_1));({});})
;
((uuid__000000000001eeaf.field_0)==(1));
})?(({({uuid__000000000001eeb0=(((LM_ListType)(uuid__000000000001eead)).field_2);({});})
;
1;
})?({uuid__000000000001eeae=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeae==(1));
})?(({({({({uuid__000000000001eeb1=(((LM_ListType)(uuid__000000000001eeab)).field_2);({});})
;
({uuid__000000000001eeb2=(0);({});})
;
});
(((uuid__000000000001eeb1.field_0)==(1))?(({({({({uuid__000000000001eeb3=(*(((LM_Type)(uuid__000000000001eeb1)).field_1001));({});})
;
({uuid__000000000001eeb4=(0);({});})
;
});
(((uuid__000000000001eeb3.field_0)==(0))?(({({uuid__000000000001eeb5=(*(((LM_ListType)(uuid__000000000001eeb3)).field_1));({});})
;
((uuid__000000000001eeb5.field_0)==(1));
})?(({({uuid__000000000001eeb6=(((LM_ListType)(uuid__000000000001eeb3)).field_2);({});})
;
1;
})?({uuid__000000000001eeb4=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeb4==(1));
})?(({({uuid__000000000001eeb7=(((LM_Type)(uuid__000000000001eeb1)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eeb7,"..."));
})?({uuid__000000000001eeb2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeb2==(1));
})?({uuid__000000000001eeac=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeac==(1));
})?(({({uuid__000000000001eeb8=(((LM_Type)(uuid__000000000001eea9)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eeb8,"Cons"));
})?({uuid__000000000001eeaa=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeaa==(1));
})?({uuid__000000000001ee9e=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ee9e==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001eeb6,uuid__000000000001eea7))?({uuid__000000000001edfe=(and_CL_ArrowTContextConsTContextTContext((unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eeb0,uuid__000000000001eea6)),(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eeb6,uuid__000000000001eea7))));({});}):({uuid__000000000001edfe=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eeb0,uuid__000000000001edfd));({});})):(({({({({uuid__000000000001eeb9=uuid__000000000001edff;({});})
;
({uuid__000000000001eeba=(0);({});})
;
});
(((uuid__000000000001eeb9.field_0)==(0))?(({({uuid__000000000001eebb=(((LM_TupleTypeType)(uuid__000000000001eeb9)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001eebc=(((LM_TupleTypeType)(uuid__000000000001eeb9)).field_2);({});})
;
({uuid__000000000001eebd=(0);({});})
;
});
(((uuid__000000000001eebc.field_0)==(1))?(({({({({uuid__000000000001eebe=(*(((LM_Type)(uuid__000000000001eebc)).field_1001));({});})
;
({uuid__000000000001eebf=(0);({});})
;
});
(((uuid__000000000001eebe.field_0)==(0))?(({({({({uuid__000000000001eec0=(*(((LM_ListType)(uuid__000000000001eebe)).field_1));({});})
;
({uuid__000000000001eec1=(0);({});})
;
});
(((uuid__000000000001eec0.field_0)==(0))?(({({uuid__000000000001eec2=(*(((LM_ListType)(uuid__000000000001eec0)).field_1));({});})
;
((uuid__000000000001eec2.field_0)==(1));
})?(({({uuid__000000000001eec3=(((LM_ListType)(uuid__000000000001eec0)).field_2);({});})
;
1;
})?({uuid__000000000001eec1=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eec1==(1));
})?(({({({({uuid__000000000001eec4=(((LM_ListType)(uuid__000000000001eebe)).field_2);({});})
;
({uuid__000000000001eec5=(0);({});})
;
});
(((uuid__000000000001eec4.field_0)==(1))?(({({({({uuid__000000000001eec6=(*(((LM_Type)(uuid__000000000001eec4)).field_1001));({});})
;
({uuid__000000000001eec7=(0);({});})
;
});
(((uuid__000000000001eec6.field_0)==(0))?(({({uuid__000000000001eec8=(*(((LM_ListType)(uuid__000000000001eec6)).field_1));({});})
;
((uuid__000000000001eec8.field_0)==(1));
})?(({({uuid__000000000001eec9=(((LM_ListType)(uuid__000000000001eec6)).field_2);({});})
;
1;
})?({uuid__000000000001eec7=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eec7==(1));
})?(({({uuid__000000000001eeca=(((LM_Type)(uuid__000000000001eec4)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eeca,"..."));
})?({uuid__000000000001eec5=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eec5==(1));
})?({uuid__000000000001eebf=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eebf==(1));
})?(({({uuid__000000000001eecb=(((LM_Type)(uuid__000000000001eebc)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eecb,"Cons"));
})?({uuid__000000000001eebd=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eebd==(1));
})?({uuid__000000000001eeba=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeba==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001eec9,uuid__000000000001eebb))?({uuid__000000000001edfe=(and_CL_ArrowTContextConsTContextTContext((unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eec9,uuid__000000000001eebb)),(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eec3,(t1_CL_ArrowTypeString("Nil"))))));({});}):({uuid__000000000001edfe=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eec3,uuid__000000000001eebb));({});})):(({({({({uuid__000000000001eecc=uuid__000000000001edff;({});})
;
({uuid__000000000001eecd=(0);({});})
;
});
(((uuid__000000000001eecc.field_0)==(0))?(({({({({uuid__000000000001eece=(((LM_TupleTypeType)(uuid__000000000001eecc)).field_1);({});})
;
({uuid__000000000001eecf=(0);({});})
;
});
(((uuid__000000000001eece.field_0)==(1))?(({({({({uuid__000000000001eed0=(*(((LM_Type)(uuid__000000000001eece)).field_1001));({});})
;
({uuid__000000000001eed1=(0);({});})
;
});
(((uuid__000000000001eed0.field_0)==(0))?(({({({({uuid__000000000001eed2=(*(((LM_ListType)(uuid__000000000001eed0)).field_1));({});})
;
({uuid__000000000001eed3=(0);({});})
;
});
(((uuid__000000000001eed2.field_0)==(0))?(({({uuid__000000000001eed4=(*(((LM_ListType)(uuid__000000000001eed2)).field_1));({});})
;
((uuid__000000000001eed4.field_0)==(1));
})?(({({uuid__000000000001eed5=(((LM_ListType)(uuid__000000000001eed2)).field_2);({});})
;
1;
})?({uuid__000000000001eed3=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eed3==(1));
})?(({({uuid__000000000001eed6=(((LM_ListType)(uuid__000000000001eed0)).field_2);({});})
;
1;
})?({uuid__000000000001eed1=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eed1==(1));
})?(({({uuid__000000000001eed7=(((LM_Type)(uuid__000000000001eece)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eed7,"Cons"));
})?({uuid__000000000001eecf=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eecf==(1));
})?(({({({({uuid__000000000001eed8=(((LM_TupleTypeType)(uuid__000000000001eecc)).field_2);({});})
;
({uuid__000000000001eed9=(0);({});})
;
});
(((uuid__000000000001eed8.field_0)==(1))?(({({({({uuid__000000000001eeda=(*(((LM_Type)(uuid__000000000001eed8)).field_1001));({});})
;
({uuid__000000000001eedb=(0);({});})
;
});
(((uuid__000000000001eeda.field_0)==(0))?(({({uuid__000000000001eedc=(*(((LM_ListType)(uuid__000000000001eeda)).field_1));({});})
;
((uuid__000000000001eedc.field_0)==(1));
})?(({({uuid__000000000001eedd=(((LM_ListType)(uuid__000000000001eeda)).field_2);({});})
;
1;
})?({uuid__000000000001eedb=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eedb==(1));
})?(({({uuid__000000000001eede=(((LM_Type)(uuid__000000000001eed8)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eede,"..."));
})?({uuid__000000000001eed9=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eed9==(1));
})?({uuid__000000000001eecd=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eecd==(1));
})?((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001eedd,uuid__000000000001eed6))?({uuid__000000000001edfe=(and_CL_ArrowTContextConsTContextTContext((unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001edfc,uuid__000000000001eed5)),(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eedd,uuid__000000000001eed6))));({});}):({})):(({({({({uuid__000000000001eedf=uuid__000000000001edff;({});})
;
({uuid__000000000001eee0=(0);({});})
;
});
(((uuid__000000000001eedf.field_0)==(0))?(({({({({uuid__000000000001eee1=(((LM_TupleTypeType)(uuid__000000000001eedf)).field_1);({});})
;
({uuid__000000000001eee2=(0);({});})
;
});
(((uuid__000000000001eee1.field_0)==(1))?(({({uuid__000000000001eee3=(*(((LM_Type)(uuid__000000000001eee1)).field_1001));({});})
;
((uuid__000000000001eee3.field_0)==(1));
})?(({({uuid__000000000001eee4=(((LM_Type)(uuid__000000000001eee1)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eee4,"Nil"));
})?({uuid__000000000001eee2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eee2==(1));
})?(({({({({uuid__000000000001eee5=(((LM_TupleTypeType)(uuid__000000000001eedf)).field_2);({});})
;
({uuid__000000000001eee6=(0);({});})
;
});
(((uuid__000000000001eee5.field_0)==(1))?(({({({({uuid__000000000001eee7=(*(((LM_Type)(uuid__000000000001eee5)).field_1001));({});})
;
({uuid__000000000001eee8=(0);({});})
;
});
(((uuid__000000000001eee7.field_0)==(0))?(({({uuid__000000000001eee9=(*(((LM_ListType)(uuid__000000000001eee7)).field_1));({});})
;
((uuid__000000000001eee9.field_0)==(1));
})?(({({uuid__000000000001eeea=(((LM_ListType)(uuid__000000000001eee7)).field_2);({});})
;
1;
})?({uuid__000000000001eee8=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eee8==(1));
})?(({({uuid__000000000001eeeb=(((LM_Type)(uuid__000000000001eee5)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eeeb,"..."));
})?({uuid__000000000001eee6=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eee6==(1));
})?({uuid__000000000001eee0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eee0==(1));
})?({}):(({({({({uuid__000000000001eeec=uuid__000000000001edff;({});})
;
({uuid__000000000001eeed=(0);({});})
;
});
(((uuid__000000000001eeec.field_0)==(0))?(({({uuid__000000000001eeee=(((LM_TupleTypeType)(uuid__000000000001eeec)).field_1);({});})
;
1;
})?(({({({({uuid__000000000001eeef=(((LM_TupleTypeType)(uuid__000000000001eeec)).field_2);({});})
;
({uuid__000000000001eef0=(0);({});})
;
});
(((uuid__000000000001eeef.field_0)==(1))?(({({({({uuid__000000000001eef1=(*(((LM_Type)(uuid__000000000001eeef)).field_1001));({});})
;
({uuid__000000000001eef2=(0);({});})
;
});
(((uuid__000000000001eef1.field_0)==(0))?(({({uuid__000000000001eef3=(*(((LM_ListType)(uuid__000000000001eef1)).field_1));({});})
;
((uuid__000000000001eef3.field_0)==(1));
})?(({({uuid__000000000001eef4=(((LM_ListType)(uuid__000000000001eef1)).field_2);({});})
;
1;
})?({uuid__000000000001eef2=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eef2==(1));
})?(({({uuid__000000000001eef5=(((LM_Type)(uuid__000000000001eeef)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001eef5,"..."));
})?({uuid__000000000001eef0=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eef0==(1));
})?({uuid__000000000001eeed=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeed==(1));
})?({uuid__000000000001edfe=(unify_SB_inner_CL_ArrowTContextConsTypeType(uuid__000000000001eef4,uuid__000000000001eeee));({});}):(({({({({uuid__000000000001eef6=uuid__000000000001edff;({});})
;
({uuid__000000000001eef7=(0);({});})
;
});
(((uuid__000000000001eef6.field_0)==(0))?(({({({({uuid__000000000001eef8=(((LM_TupleTypeType)(uuid__000000000001eef6)).field_1);({});})
;
({uuid__000000000001eef9=(0);({});})
;
});
(((uuid__000000000001eef8.field_0)==(1))?(({({uuid__000000000001eefa=(*(((LM_Type)(uuid__000000000001eef8)).field_1001));({});})
;
((uuid__000000000001eefa.field_0)==(1));
})?(({({uuid__000000000001eefb=(((LM_Type)(uuid__000000000001eef8)).field_1002);({});})
;
1;
})?({uuid__000000000001eef9=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eef9==(1));
})?(({({({({uuid__000000000001eefc=(((LM_TupleTypeType)(uuid__000000000001eef6)).field_2);({});})
;
({uuid__000000000001eefd=(0);({});})
;
});
(((uuid__000000000001eefc.field_0)==(1))?(({({({({uuid__000000000001eefe=(*(((LM_Type)(uuid__000000000001eefc)).field_1001));({});})
;
({uuid__000000000001eeff=(0);({});})
;
});
(((uuid__000000000001eefe.field_0)==(0))?(({({uuid__000000000001ef00=(*(((LM_ListType)(uuid__000000000001eefe)).field_1));({});})
;
((uuid__000000000001ef00.field_0)==(1));
})?(({({({({uuid__000000000001ef01=(((LM_ListType)(uuid__000000000001eefe)).field_2);({});})
;
({uuid__000000000001ef02=(0);({});})
;
});
(((uuid__000000000001ef01.field_0)==(1))?(({({uuid__000000000001ef03=(*(((LM_Type)(uuid__000000000001ef01)).field_1001));({});})
;
((uuid__000000000001ef03.field_0)==(1));
})?(({({uuid__000000000001ef04=(((LM_Type)(uuid__000000000001ef01)).field_1002);({});})
;
1;
})?({uuid__000000000001ef02=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef02==(1));
})?({uuid__000000000001eeff=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eeff==(1));
})?(({({uuid__000000000001ef05=(((LM_Type)(uuid__000000000001eefc)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ef05,"GT"));
})?({uuid__000000000001eefd=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eefd==(1));
})?({uuid__000000000001eef7=(1);({});}):({})):({})):({}));
});
(uuid__000000000001eef7==(1));
})?(((to_SB_i64_CL_ArrowI64String(uuid__000000000001eefb))>(to_SB_i64_CL_ArrowI64String(uuid__000000000001ef04)))?({uuid__000000000001edfe=({LM_TContext rvalue={1};rvalue;});({});}):({})):(({({({({uuid__000000000001ef06=uuid__000000000001edff;({});})
;
({uuid__000000000001ef07=(0);({});})
;
});
(((uuid__000000000001ef06.field_0)==(0))?(({({({({uuid__000000000001ef08=(((LM_TupleTypeType)(uuid__000000000001ef06)).field_1);({});})
;
({uuid__000000000001ef09=(0);({});})
;
});
(((uuid__000000000001ef08.field_0)==(1))?(({({uuid__000000000001ef0a=(*(((LM_Type)(uuid__000000000001ef08)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ef0b=(((LM_Type)(uuid__000000000001ef08)).field_1002);({});})
;
1;
})?({uuid__000000000001ef09=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef09==(1));
})?(({({({({uuid__000000000001ef0c=(((LM_TupleTypeType)(uuid__000000000001ef06)).field_2);({});})
;
({uuid__000000000001ef0d=(0);({});})
;
});
(((uuid__000000000001ef0c.field_0)==(1))?(({({uuid__000000000001ef0e=(*(((LM_Type)(uuid__000000000001ef0c)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ef0f=(((LM_Type)(uuid__000000000001ef0c)).field_1002);({});})
;
1;
})?({uuid__000000000001ef0d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef0d==(1));
})?({uuid__000000000001ef07=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef07==(1));
})?((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ef0f,uuid__000000000001ef0b))?({uuid__000000000001edfe=(unify_CL_ArrowTContextConsListTypeListType(uuid__000000000001ef0e,uuid__000000000001ef0a));({});}):({})):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/unify.lm Line: 12 Column: 5"))))))))))))))))));
});
});
uuid__000000000001edfe;
});}
#line 153 "SRC/unify.lm"
LM_TContext unify_CL_ArrowTContextConsListTypeListType(LM_ListType uuid__000000000001ef10,LM_ListType uuid__000000000001ef11){LM_TContext uuid__000000000001ef12;
LM_TupleListTypeListType uuid__000000000001ef13;
LM_TupleListTypeListType uuid__000000000001ef14;
char uuid__000000000001ef15;
LM_ListType uuid__000000000001ef16;
LM_ListType uuid__000000000001ef17;
LM_TupleListTypeListType uuid__000000000001ef18;
char uuid__000000000001ef19;
LM_ListType uuid__000000000001ef1a;
char uuid__000000000001ef1b;
LM_ListType uuid__000000000001ef1c;
LM_Type uuid__000000000001ef1d;
LM_ListType uuid__000000000001ef1e;
char uuid__000000000001ef1f;
LM_ListType uuid__000000000001ef20;
LM_Type uuid__000000000001ef21;
return ({({({uuid__000000000001ef12=({LM_TContext rvalue={2};rvalue;});({});})
;
({({uuid__000000000001ef13=({LM_TupleListTypeListType rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ef11;rvalue.field_2=uuid__000000000001ef10;rvalue;});({});})
;
(({({({({uuid__000000000001ef14=uuid__000000000001ef13;({});})
;
({uuid__000000000001ef15=(0);({});})
;
});
(((uuid__000000000001ef14.field_0)==(0))?(({({uuid__000000000001ef16=(((LM_TupleListTypeListType)(uuid__000000000001ef14)).field_1);({});})
;
((uuid__000000000001ef16.field_0)==(1));
})?(({({uuid__000000000001ef17=(((LM_TupleListTypeListType)(uuid__000000000001ef14)).field_2);({});})
;
((uuid__000000000001ef17.field_0)==(1));
})?({uuid__000000000001ef15=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef15==(1));
})?({uuid__000000000001ef12=({LM_TContext rvalue={1};rvalue;});({});}):(({({({({uuid__000000000001ef18=uuid__000000000001ef13;({});})
;
({uuid__000000000001ef19=(0);({});})
;
});
(((uuid__000000000001ef18.field_0)==(0))?(({({({({uuid__000000000001ef1a=(((LM_TupleListTypeListType)(uuid__000000000001ef18)).field_1);({});})
;
({uuid__000000000001ef1b=(0);({});})
;
});
(((uuid__000000000001ef1a.field_0)==(0))?(({({uuid__000000000001ef1c=(*(((LM_ListType)(uuid__000000000001ef1a)).field_1));({});})
;
1;
})?(({({uuid__000000000001ef1d=(((LM_ListType)(uuid__000000000001ef1a)).field_2);({});})
;
1;
})?({uuid__000000000001ef1b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef1b==(1));
})?(({({({({uuid__000000000001ef1e=(((LM_TupleListTypeListType)(uuid__000000000001ef18)).field_2);({});})
;
({uuid__000000000001ef1f=(0);({});})
;
});
(((uuid__000000000001ef1e.field_0)==(0))?(({({uuid__000000000001ef20=(*(((LM_ListType)(uuid__000000000001ef1e)).field_1));({});})
;
1;
})?(({({uuid__000000000001ef21=(((LM_ListType)(uuid__000000000001ef1e)).field_2);({});})
;
1;
})?({uuid__000000000001ef1f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef1f==(1));
})?({uuid__000000000001ef19=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef19==(1));
})?({({uuid__000000000001ef12=(unify_CL_ArrowTContextConsTypeType(uuid__000000000001ef21,uuid__000000000001ef1d));({});});
((non_SB_zero_CL_ArrowU64TContext(uuid__000000000001ef12))?({uuid__000000000001ef12=(union_CL_ArrowTContextConsTContextTContext(uuid__000000000001ef12,(unify_CL_ArrowTContextConsListTypeListType(uuid__000000000001ef20,uuid__000000000001ef1c))));({});}):({}));
}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/unify.lm Line: 155 Column: 5")))));
});
});
uuid__000000000001ef12;
});}
#line 2 "SRC/tctx-substitute.lm"
LM_ListType substitute_CL_ArrowListTypeConsListTypeTContext(LM_TContext uuid__000000000001ef22,LM_ListType uuid__000000000001ef23){LM_ListType uuid__000000000001ef24;
LM_ListType uuid__000000000001ef25;
LM_ListType uuid__000000000001ef26;
char uuid__000000000001ef27;
LM_ListType uuid__000000000001ef28;
LM_Type uuid__000000000001ef29;
return ({({({uuid__000000000001ef24=uuid__000000000001ef23;({});})
;
(({({uuid__000000000001ef25=uuid__000000000001ef24;({});})
;
((uuid__000000000001ef25.field_0)==(1));
})?({}):(({({({({uuid__000000000001ef26=uuid__000000000001ef24;({});})
;
({uuid__000000000001ef27=(0);({});})
;
});
(((uuid__000000000001ef26.field_0)==(0))?(({({uuid__000000000001ef28=(*(((LM_ListType)(uuid__000000000001ef26)).field_1));({});})
;
1;
})?(({({uuid__000000000001ef29=(((LM_ListType)(uuid__000000000001ef26)).field_2);({});})
;
1;
})?({uuid__000000000001ef27=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef27==(1));
})?({uuid__000000000001ef23=(cons_CL_ArrowListTypeConsListTypeType((substitute_CL_ArrowTypeConsTypeTContext(uuid__000000000001ef22,uuid__000000000001ef29)),(substitute_CL_ArrowListTypeConsListTypeTContext(uuid__000000000001ef22,uuid__000000000001ef28))));({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/tctx-substitute.lm Line: 3 Column: 5"))));
});
uuid__000000000001ef23;
});}
#line 16 "SRC/tctx-substitute.lm"
LM_Type substitute_CL_ArrowTypeConsTypeTContext(LM_TContext uuid__000000000001ef2a,LM_Type uuid__000000000001ef2b){LM_Type uuid__000000000001ef2c;
LM_Type uuid__000000000001ef2d;
char uuid__000000000001ef2e;
char* uuid__000000000001ef2f;
LM_TContext uuid__000000000001ef30;
LM_TContext uuid__000000000001ef31;
LM_TContext uuid__000000000001ef32;
char uuid__000000000001ef33;
LM_Type uuid__000000000001ef34;
char* uuid__000000000001ef35;
LM_TContext uuid__000000000001ef36;
LM_Type uuid__000000000001ef37;
char uuid__000000000001ef38;
LM_ListType uuid__000000000001ef39;
char* uuid__000000000001ef3a;
LM_Type uuid__000000000001ef3b;
char uuid__000000000001ef3c;
LM_Type uuid__000000000001ef3d;
LM_Type uuid__000000000001ef3e;
return ({({({uuid__000000000001ef2c=uuid__000000000001ef2b;({});})
;
(({({({({uuid__000000000001ef2d=uuid__000000000001ef2c;({});})
;
({uuid__000000000001ef2e=(0);({});})
;
});
(((uuid__000000000001ef2d.field_0)==(2))?(({({uuid__000000000001ef2f=(((LM_Type)(uuid__000000000001ef2d)).field_2001);({});})
;
1;
})?({uuid__000000000001ef2e=(1);({});}):({})):({}));
});
(uuid__000000000001ef2e==(1));
})?({while((non_SB_zero_CL_ArrowU64TContext(uuid__000000000001ef2a))){((void)(({({uuid__000000000001ef30=uuid__000000000001ef2a;({});})
;
(({({uuid__000000000001ef31=uuid__000000000001ef30;({});})
;
((uuid__000000000001ef31.field_0)==(1));
})?({uuid__000000000001ef2a=({LM_TContext rvalue={2};rvalue;});({});}):(({({({({uuid__000000000001ef32=uuid__000000000001ef30;({});})
;
({uuid__000000000001ef33=(0);({});})
;
});
(((uuid__000000000001ef32.field_0)==(0))?(1?(({({uuid__000000000001ef34=(((LM_TContext)(uuid__000000000001ef32)).field_2);({});})
;
1;
})?(({({uuid__000000000001ef35=(((LM_TContext)(uuid__000000000001ef32)).field_3);({});})
;
1;
})?(({({uuid__000000000001ef36=(*(((LM_TContext)(uuid__000000000001ef32)).field_4));({});})
;
1;
})?({uuid__000000000001ef33=(1);({});}):({})):({})):({})):({})):({}));
});
(uuid__000000000001ef33==(1));
})?((_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ef35,uuid__000000000001ef2f))?({({uuid__000000000001ef2b=uuid__000000000001ef34;({});});
({uuid__000000000001ef2a=({LM_TContext rvalue={2};rvalue;});({});});
}):({uuid__000000000001ef2a=uuid__000000000001ef36;({});})):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/tctx-substitute.lm Line: 20 Column: 34"))));
})));};}):(({({({({uuid__000000000001ef37=uuid__000000000001ef2c;({});})
;
({uuid__000000000001ef38=(0);({});})
;
});
(((uuid__000000000001ef37.field_0)==(1))?(({({uuid__000000000001ef39=(*(((LM_Type)(uuid__000000000001ef37)).field_1001));({});})
;
1;
})?(({({uuid__000000000001ef3a=(((LM_Type)(uuid__000000000001ef37)).field_1002);({});})
;
1;
})?({uuid__000000000001ef38=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef38==(1));
})?({uuid__000000000001ef2b=({LM_Type rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_ListTypeListType((substitute_CL_ArrowListTypeConsListTypeTContext(uuid__000000000001ef2a,uuid__000000000001ef39))));rvalue.field_1002=uuid__000000000001ef3a;rvalue;});({});}):(({({({({uuid__000000000001ef3b=uuid__000000000001ef2c;({});})
;
({uuid__000000000001ef3c=(0);({});})
;
});
(((uuid__000000000001ef3b.field_0)==(0))?(({({uuid__000000000001ef3d=(*(((LM_Type)(uuid__000000000001ef3b)).field_1));({});})
;
1;
})?(({({uuid__000000000001ef3e=(*(((LM_Type)(uuid__000000000001ef3b)).field_2));({});})
;
1;
})?({uuid__000000000001ef3c=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef3c==(1));
})?({uuid__000000000001ef2b=({LM_Type rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_TypeType((substitute_CL_ArrowTypeConsTypeTContext(uuid__000000000001ef2a,uuid__000000000001ef3d))));rvalue.field_2=(close_CL_ArrowArray_QM_TypeType((substitute_CL_ArrowTypeConsTypeTContext(uuid__000000000001ef2a,uuid__000000000001ef3e))));rvalue;});({});}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/tctx-substitute.lm Line: 17 Column: 5"))))));
});
uuid__000000000001ef2b;
});}
#line 3 "SRC/tctx-union.lm"
LM_TContext union_CL_ArrowTContextConsTContextTContext(LM_TContext uuid__000000000001ef3f,LM_TContext uuid__000000000001ef40){LM_TContext uuid__000000000001ef41;
LM_TContext uuid__000000000001ef42;
LM_TContext uuid__000000000001ef43;
LM_TContext uuid__000000000001ef44;
char uuid__000000000001ef45;
LM_AST uuid__000000000001ef46;
LM_Type uuid__000000000001ef47;
char* uuid__000000000001ef48;
LM_TContext uuid__000000000001ef49;
return ({({({uuid__000000000001ef41=uuid__000000000001ef40;({});})
;
(({({uuid__000000000001ef42=uuid__000000000001ef41;({});})
;
((uuid__000000000001ef42.field_0)==(1));
})?({uuid__000000000001ef40=({LM_TContext rvalue={2};rvalue;});({});}):(({({uuid__000000000001ef43=uuid__000000000001ef41;({});})
;
((uuid__000000000001ef43.field_0)==(2));
})?({}):(({({({({uuid__000000000001ef44=uuid__000000000001ef41;({});})
;
({uuid__000000000001ef45=(0);({});})
;
});
(((uuid__000000000001ef44.field_0)==(0))?(({({uuid__000000000001ef46=(((LM_TContext)(uuid__000000000001ef44)).field_1);({});})
;
1;
})?(({({uuid__000000000001ef47=(((LM_TContext)(uuid__000000000001ef44)).field_2);({});})
;
1;
})?(({({uuid__000000000001ef48=(((LM_TContext)(uuid__000000000001ef44)).field_3);({});})
;
1;
})?(({({uuid__000000000001ef49=(*(((LM_TContext)(uuid__000000000001ef44)).field_4));({});})
;
1;
})?({uuid__000000000001ef45=(1);({});}):({})):({})):({})):({})):({}));
});
(uuid__000000000001ef45==(1));
})?({({uuid__000000000001ef3f=(union_CL_ArrowTContextConsTContextTContext(uuid__000000000001ef3f,uuid__000000000001ef49));({});});
({uuid__000000000001ef3f=({LM_TContext rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ef46;rvalue.field_2=uuid__000000000001ef47;rvalue.field_3=uuid__000000000001ef48;rvalue.field_4=(close_CL_ArrowArray_QM_TContextTContext(uuid__000000000001ef3f));rvalue;});({});});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/tctx-union.lm Line: 4 Column: 5")))));
});
uuid__000000000001ef3f;
});}
#line 4 "SRC/tctx-and.lm"
LM_TContext and_CL_ArrowTContextConsTContextTContext(LM_TContext uuid__000000000001ef4a,LM_TContext uuid__000000000001ef4b){LM_TupleTContextTContext uuid__000000000001ef4c;
LM_TupleTContextTContext uuid__000000000001ef5f;
char uuid__000000000001ef60;
LM_TContext uuid__000000000001ef61;
LM_TupleTContextTContext uuid__000000000001ef62;
char uuid__000000000001ef63;
LM_TContext uuid__000000000001ef64;
return ({({({uuid__000000000001ef4c=({LM_TupleTContextTContext rvalue={.field_0=0};rvalue.field_1=uuid__000000000001ef4b;rvalue.field_2=uuid__000000000001ef4a;rvalue;});({});})
;
(({({({({uuid__000000000001ef5f=uuid__000000000001ef4c;({});})
;
({uuid__000000000001ef60=(0);({});})
;
});
(((uuid__000000000001ef5f.field_0)==(0))?(1?(({({uuid__000000000001ef61=(((LM_TupleTContextTContext)(uuid__000000000001ef5f)).field_2);({});})
;
((uuid__000000000001ef61.field_0)==(2));
})?({uuid__000000000001ef60=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef60==(1));
})?({uuid__000000000001ef4a=({LM_TContext rvalue={2};rvalue;});({});}):(({({({({uuid__000000000001ef62=uuid__000000000001ef4c;({});})
;
({uuid__000000000001ef63=(0);({});})
;
});
(((uuid__000000000001ef62.field_0)==(0))?(({({uuid__000000000001ef64=(((LM_TupleTContextTContext)(uuid__000000000001ef62)).field_1);({});})
;
((uuid__000000000001ef64.field_0)==(2));
})?(1?({uuid__000000000001ef63=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef63==(1));
})?({uuid__000000000001ef4a=({LM_TContext rvalue={2};rvalue;});({});}):(1?({uuid__000000000001ef4a=(union_CL_ArrowTContextConsTContextTContext(uuid__000000000001ef4a,uuid__000000000001ef4b));({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/tctx-and.lm Line: 5 Column: 5")))));
});
uuid__000000000001ef4a;
});}
#line 4 "SRC/quick-prop.lsts"
LM_TupleU64String _DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(LM_Type uuid__000000000001ef65){LM_Type uuid__000000000001ef66;
LM_Type uuid__000000000001ef67;
char uuid__000000000001ef68;
LM_ListType uuid__000000000001ef69;
LM_ListType uuid__000000000001ef6a;
LM_Type uuid__000000000001ef6b;
LM_ListType uuid__000000000001ef6c;
char* uuid__000000000001ef6d;
LM_TupleU64String uuid__000000000001ef6e;
LM_Type uuid__000000000001ef8d;
char uuid__000000000001ef8e;
char* uuid__000000000001ef8f;
LM_Type uuid__000000000001efa2;
char uuid__000000000001efa3;
LM_ListType uuid__000000000001efa4;
char* uuid__000000000001efa5;
LM_Type uuid__000000000001efb8;
LM_Type uuid__000000000001efcb;
LM_Type uuid__000000000001efde;
char uuid__000000000001efdf;
LM_Type uuid__000000000001efe0;
LM_Type uuid__000000000001efe1;
LM_TupleU64String uuid__000000000001efe2;
return ({({uuid__000000000001ef66=uuid__000000000001ef65;({});})
;
(({({({({uuid__000000000001ef67=uuid__000000000001ef66;({});})
;
({uuid__000000000001ef68=(0);({});})
;
});
(((uuid__000000000001ef67.field_0)==(1))?(({({uuid__000000000001ef69=(*(((LM_Type)(uuid__000000000001ef67)).field_1001));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ef69))?(1?({({uuid__000000000001ef6a=(tail_CL_ArrowListTypeListType(uuid__000000000001ef69));({});})
;
((_DT_has_SB_head_CL_ArrowU64ListType(uuid__000000000001ef6a))?(({({uuid__000000000001ef6b=(head_CL_ArrowTypeListType(uuid__000000000001ef6a));({});})
;
1;
})?({({uuid__000000000001ef6c=(tail_CL_ArrowListTypeListType(uuid__000000000001ef6a));({});})
;
((uuid__000000000001ef6c.field_0)==(1));
}):0):0);
}):0):0);
})?(({({uuid__000000000001ef6d=(((LM_Type)(uuid__000000000001ef67)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ef6d,"Array"));
})?({uuid__000000000001ef68=(1);({});}):({})):({})):({}));
});
(uuid__000000000001ef68==(1));
})?({({uuid__000000000001ef6e=(_DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(uuid__000000000001ef6b));({});})
;
({LM_TupleU64String rvalue={.field_0=0};rvalue.field_1=((uuid__000000000001ef6e.field_1)+(1000));rvalue.field_2=(uuid__000000000001ef6e.field_2);rvalue;});
}):(({({({({uuid__000000000001ef8d=uuid__000000000001ef66;({});})
;
({uuid__000000000001ef8e=(0);({});})
;
});
(((uuid__000000000001ef8d.field_0)==(1))?(({({uuid__000000000001ef8f=(((LM_Type)(uuid__000000000001ef8d)).field_1002);({});})
;
(_EQ__EQ__CL_ArrowU64ConsStringString(uuid__000000000001ef8f,"Sized"));
})?({uuid__000000000001ef8e=(1);({});}):({})):({}));
});
(uuid__000000000001ef8e==(1));
})?({LM_TupleU64String rvalue={.field_0=0};rvalue.field_1=(9999999);rvalue.field_2="";rvalue;}):(({({({({uuid__000000000001efa2=uuid__000000000001ef66;({});})
;
({uuid__000000000001efa3=(0);({});})
;
});
(((uuid__000000000001efa2.field_0)==(1))?(({({uuid__000000000001efa4=(*(((LM_Type)(uuid__000000000001efa2)).field_1001));({});})
;
1;
})?(({({uuid__000000000001efa5=(((LM_Type)(uuid__000000000001efa2)).field_1002);({});})
;
1;
})?({uuid__000000000001efa3=(1);({});}):({})):({})):({}));
});
(uuid__000000000001efa3==(1));
})?({LM_TupleU64String rvalue={.field_0=0};rvalue.field_1=(_DT_length_CL_ArrowU64ListType(uuid__000000000001efa4));rvalue.field_2=uuid__000000000001efa5;rvalue;}):(({({uuid__000000000001efb8=uuid__000000000001ef66;({});})
;
((uuid__000000000001efb8.field_0)==(3));
})?({LM_TupleU64String rvalue={.field_0=0};rvalue.field_1=(0);rvalue.field_2="?";rvalue;}):(({({uuid__000000000001efcb=uuid__000000000001ef66;({});})
;
((uuid__000000000001efcb.field_0)==(2));
})?({LM_TupleU64String rvalue={.field_0=0};rvalue.field_1=(9999999);rvalue.field_2="";rvalue;}):(({({({({uuid__000000000001efde=uuid__000000000001ef66;({});})
;
({uuid__000000000001efdf=(0);({});})
;
});
(((uuid__000000000001efde.field_0)==(0))?(({({uuid__000000000001efe0=(*(((LM_Type)(uuid__000000000001efde)).field_1));({});})
;
1;
})?(({({uuid__000000000001efe1=(*(((LM_Type)(uuid__000000000001efde)).field_2));({});})
;
1;
})?({uuid__000000000001efdf=(1);({});}):({})):({})):({}));
});
(uuid__000000000001efdf==(1));
})?({({uuid__000000000001efe2=(_DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(uuid__000000000001efe1));({});})
;
(((uuid__000000000001efe2.field_1)==(9999999))?(_DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(uuid__000000000001efe0)):uuid__000000000001efe2);
}):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/quick-prop.lsts Line: 5 Column: 4"));LM_TupleU64String rvalue;rvalue;})))))));
});}
#line 21 "SRC/quick-prop.lsts"
void add_SB_quick_SB_prop_CL_ArrowNilConsTypeConsTypeType(LM_Type uuid__000000000001effb,LM_Type uuid__000000000001effc,LM_Type uuid__000000000001effd){LM_TupleU64String uuid__000000000001effe;
LM_ListTupleTypeType uuid__000000000001f00b;
({({({({uuid__000000000001effe=(_DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(uuid__000000000001effb));({});})
;
({uuid__000000000001f00b=(_DT_lookup_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String,uuid__000000000001effe,({LM_ListTupleTypeType rvalue={1};rvalue;})));({});})
;
});
({uuid__000000000001f00b=(cons_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeTupleTypeType(({LM_TupleTypeType rvalue={.field_0=0};rvalue.field_1=uuid__000000000001effd;rvalue.field_2=uuid__000000000001effc;rvalue;}),uuid__000000000001f00b));({});});
});
({quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String=(_DT_bind_CL_ArrowHashtableEqListTupleTypeTypeTupleU64StringConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String,uuid__000000000001effe,uuid__000000000001f00b));({});});
});}
#line 28 "SRC/quick-prop.lsts"
LM_Type enrich_SB_quick_SB_prop_CL_ArrowTypeType(LM_Type uuid__000000000001f01e){return (enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(uuid__000000000001f01e,uuid__000000000001f01e));}
#line 32 "SRC/quick-prop.lsts"
LM_Type enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(LM_Type uuid__000000000001f01f,LM_Type uuid__000000000001f020){LM_Type uuid__000000000001f021;
LM_Type uuid__000000000001f022;
char uuid__000000000001f023;
LM_ListType uuid__000000000001f024;
char* uuid__000000000001f025;
LM_ListType uuid__000000000001f026;
LM_ListType uuid__000000000001f028;
LM_ListType uuid__000000000001f029;
LM_ListType uuid__000000000001f02a;
char uuid__000000000001f02b;
LM_ListType uuid__000000000001f02c;
LM_Type uuid__000000000001f02d;
LM_ListTupleTypeType uuid__000000000001f02e;
LM_ListTupleTypeType uuid__000000000001f03b;
LM_ListTupleTypeType uuid__000000000001f03c;
char uuid__000000000001f03d;
LM_ListTupleTypeType uuid__000000000001f03e;
LM_TupleTypeType uuid__000000000001f03f;
char uuid__000000000001f040;
LM_Type uuid__000000000001f041;
LM_Type uuid__000000000001f042;
LM_TContext uuid__000000000001f043;
LM_Type uuid__000000000001f044;
LM_Type uuid__000000000001f045;
char uuid__000000000001f046;
LM_Type uuid__000000000001f047;
LM_Type uuid__000000000001f048;
LM_Type uuid__000000000001f049;
LM_Type uuid__000000000001f04a;
return ({({({uuid__000000000001f021=uuid__000000000001f020;({});})
;
(({({({({uuid__000000000001f022=uuid__000000000001f021;({});})
;
({uuid__000000000001f023=(0);({});})
;
});
(((uuid__000000000001f022.field_0)==(1))?(({({uuid__000000000001f024=(*(((LM_Type)(uuid__000000000001f022)).field_1001));({});})
;
1;
})?(({({uuid__000000000001f025=(((LM_Type)(uuid__000000000001f022)).field_1002);({});})
;
1;
})?({uuid__000000000001f023=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f023==(1));
})?({(((_DT_length_CL_ArrowU64ListType(uuid__000000000001f024))>(0))?({({({uuid__000000000001f026=({LM_ListType rvalue={1};rvalue;});({});})
;
({({uuid__000000000001f028=(_DT_reverse_CL_ArrowListTypeListType(uuid__000000000001f024));({});})
;
({while((non_SB_zero_CL_ArrowU64ListType(uuid__000000000001f028))){((void)(({({uuid__000000000001f029=uuid__000000000001f028;({});})
;
(({({({({uuid__000000000001f02a=uuid__000000000001f029;({});})
;
({uuid__000000000001f02b=(0);({});})
;
});
(((uuid__000000000001f02a.field_0)==(0))?(({({uuid__000000000001f02c=(*(((LM_ListType)(uuid__000000000001f02a)).field_1));({});})
;
1;
})?(({({uuid__000000000001f02d=(((LM_ListType)(uuid__000000000001f02a)).field_2);({});})
;
1;
})?({uuid__000000000001f02b=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f02b==(1));
})?({({uuid__000000000001f026=(cons_CL_ArrowListTypeConsListTypeType((enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(uuid__000000000001f01f,uuid__000000000001f02d)),uuid__000000000001f026));({});});
({uuid__000000000001f028=uuid__000000000001f02c;({});});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/quick-prop.lsts Line: 37 Column: 13")));
})));};});
});
});
({uuid__000000000001f020=({LM_Type rvalue={.field_0=1};rvalue.field_1001=(close_CL_ArrowArray_QM_ListTypeListType(uuid__000000000001f026));rvalue.field_1002=uuid__000000000001f025;rvalue;});({});});
}):({}));
({({uuid__000000000001f02e=(_DT_lookup_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String,(_DT_ground_SB_tag_SB_and_SB_arity_CL_ArrowTupleU64StringType(uuid__000000000001f020)),({LM_ListTupleTypeType rvalue={1};rvalue;})));({});})
;
({while((non_SB_zero_CL_ArrowU64ListTupleTypeType(uuid__000000000001f02e))){((void)(({({uuid__000000000001f03b=uuid__000000000001f02e;({});})
;
(({({({({uuid__000000000001f03c=uuid__000000000001f03b;({});})
;
({uuid__000000000001f03d=(0);({});})
;
});
(((uuid__000000000001f03c.field_0)==(0))?(({({uuid__000000000001f03e=(*(((LM_ListTupleTypeType)(uuid__000000000001f03c)).field_1));({});})
;
1;
})?(({({({({uuid__000000000001f03f=(((LM_ListTupleTypeType)(uuid__000000000001f03c)).field_2);({});})
;
({uuid__000000000001f040=(0);({});})
;
});
(((uuid__000000000001f03f.field_0)==(0))?(({({uuid__000000000001f041=(((LM_TupleTypeType)(uuid__000000000001f03f)).field_1);({});})
;
1;
})?(({({uuid__000000000001f042=(((LM_TupleTypeType)(uuid__000000000001f03f)).field_2);({});})
;
1;
})?({uuid__000000000001f040=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f040==(1));
})?({uuid__000000000001f03d=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f03d==(1));
})?({((can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001f042,uuid__000000000001f01f))?({({({uuid__000000000001f043=(unify_CL_ArrowTContextConsTypeType(uuid__000000000001f042,uuid__000000000001f01f));({});})
;
({uuid__000000000001f044=(substitute_CL_ArrowTypeConsTypeTContext(uuid__000000000001f043,uuid__000000000001f041));({});})
;
});
((!(can_SB_unify_CL_ArrowU64ConsTypeType(uuid__000000000001f044,uuid__000000000001f01f)))?({uuid__000000000001f020=({LM_Type rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001f044));rvalue.field_2=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001f020));rvalue;});({});}):({}));
}):({}));
({uuid__000000000001f02e=uuid__000000000001f03e;({});});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/quick-prop.lsts Line: 42 Column: 10")));
})));};});
});
}):(({({({({uuid__000000000001f045=uuid__000000000001f021;({});})
;
({uuid__000000000001f046=(0);({});})
;
});
(((uuid__000000000001f045.field_0)==(0))?(({({uuid__000000000001f047=(*(((LM_Type)(uuid__000000000001f045)).field_1));({});})
;
1;
})?(({({uuid__000000000001f048=(*(((LM_Type)(uuid__000000000001f045)).field_2));({});})
;
1;
})?({uuid__000000000001f046=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f046==(1));
})?({({({uuid__000000000001f049=(enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(uuid__000000000001f01f,uuid__000000000001f048));({});})
;
({uuid__000000000001f04a=(enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(uuid__000000000001f01f,uuid__000000000001f047));({});})
;
});
(((!(is_CL_ArrowU64ConsTypeType(uuid__000000000001f048,uuid__000000000001f049)))||(!(is_CL_ArrowU64ConsTypeType(uuid__000000000001f047,uuid__000000000001f04a))))?({uuid__000000000001f020=({LM_Type rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001f04a));rvalue.field_2=(close_CL_ArrowArray_QM_TypeType(uuid__000000000001f049));rvalue;});({});}):({}));
}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: SRC/quick-prop.lsts Line: 33 Column: 4")))));
});
uuid__000000000001f020;
});}
#line 2 "PLATFORM/C/LIB/print.lsts"
void print_CL_ArrowNilU64(unsigned long uuid__000000000001f04b){(print_CL_ArrowNilConsU64IO_CL__CL_File(stdout,uuid__000000000001f04b));}
#line 22 "PLATFORM/C/LIB/array.lm"
LM_S* close_CL_ArrowArray_QM_SS(LM_S uuid__000000000001f04c){LM_S* uuid__000000000001f04d;
return ({({({uuid__000000000001f04d=((LM_S*)((malloc((sizeof(LM_S))))));({});})
;
(uuid__000000000001f04d[(0)]=uuid__000000000001f04c);
});
uuid__000000000001f04d;
});}
#line 3 "PLATFORM/C/LIB/sized.lm"
unsigned long hash_CL_ArrowU64U64(unsigned long uuid__000000000001f04e){unsigned long uuid__000000000001f04f;
unsigned long uuid__000000000001f050;
return ({({({({({({({uuid__000000000001f04f=(0);({});})
;
({uuid__000000000001f050=(0);({});})
;
});
({while((uuid__000000000001f04f<((sizeof(unsigned long))/(8)))){((void)(({({({({uuid__000000000001f050=(uuid__000000000001f050+(((unsigned long*)((&uuid__000000000001f04e)))[uuid__000000000001f04f]));({});});
({uuid__000000000001f04f=(uuid__000000000001f04f+(1));({});});
});
({uuid__000000000001f050=(uuid__000000000001f050+(uuid__000000000001f050<<(10)));({});});
});
({uuid__000000000001f050=(uuid__000000000001f050^(uuid__000000000001f050>>(6)));({});});
})));};});
});
({uuid__000000000001f050=(uuid__000000000001f050+(uuid__000000000001f050<<(3)));({});});
});
({uuid__000000000001f050=(uuid__000000000001f050^(uuid__000000000001f050>>(11)));({});});
});
({uuid__000000000001f050=(uuid__000000000001f050+(uuid__000000000001f050<<(15)));({});});
});
uuid__000000000001f050;
});}
#line 16 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsStringString(char* uuid__000000000001f051,char* uuid__000000000001f052){return (_EQ__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsStringString(uuid__000000000001f051,uuid__000000000001f052)),({LM_Ord rvalue={1};rvalue;})));}
#line 16 "PLATFORM/C/LIB/cmp.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001f053,LM_SmartString uuid__000000000001f054){return (_EQ__EQ__CL_ArrowU64ConsOrdOrd((cmp_CL_ArrowOrdConsSmartStringSmartString(uuid__000000000001f053,uuid__000000000001f054)),({LM_Ord rvalue={1};rvalue;})));}
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsSmartStringSmartString(LM_SmartString uuid__000000000001f055,LM_SmartString uuid__000000000001f056){unsigned long uuid__000000000001f057;
unsigned long uuid__000000000001f058;
char uuid__000000000001f059;
char uuid__000000000001f05a;
return ({({({({uuid__000000000001f057=(0);({});})
;
({uuid__000000000001f058=(1);({});})
;
});
({while((uuid__000000000001f057<(sizeof(LM_SmartString)))){((void)(({({({({uuid__000000000001f059=(((char*)((&uuid__000000000001f055)))[uuid__000000000001f057]);({});})
;
({uuid__000000000001f05a=(((char*)((&uuid__000000000001f056)))[uuid__000000000001f057]);({});})
;
});
((uuid__000000000001f059==uuid__000000000001f05a)?({}):({uuid__000000000001f058=(0);({});}));
});
({uuid__000000000001f057=(uuid__000000000001f057+(1));({});});
})));};});
});
uuid__000000000001f058;
});}
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListString(LM_ListString uuid__000000000001f05b){return ((uuid__000000000001f05b.field_0)!=(({LM_ListString rvalue={1};rvalue;}).field_0));}
#line 31 "PLATFORM/C/LIB/list.lm"
LM_ListString tail_CL_ArrowListStringListString(LM_ListString uuid__000000000001f05d){LM_ListString uuid__000000000001f05e;
LM_ListString uuid__000000000001f05f;
LM_ListString uuid__000000000001f060;
char uuid__000000000001f061;
LM_ListString uuid__000000000001f062;
char* uuid__000000000001f063;
return ({({({uuid__000000000001f05e=uuid__000000000001f05d;({});})
;
(({({uuid__000000000001f05f=uuid__000000000001f05e;({});})
;
((uuid__000000000001f05f.field_0)==(1));
})?(fail_CL_ArrowNeverString("List::head is fallible\n")):(({({({({uuid__000000000001f060=uuid__000000000001f05e;({});})
;
({uuid__000000000001f061=(0);({});})
;
});
(((uuid__000000000001f060.field_0)==(0))?(({({uuid__000000000001f062=(*(((LM_ListString)(uuid__000000000001f060)).field_1));({});})
;
1;
})?(({({uuid__000000000001f063=(((LM_ListString)(uuid__000000000001f060)).field_2);({});})
;
1;
})?({uuid__000000000001f061=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f061==(1));
})?({uuid__000000000001f05d=uuid__000000000001f062;({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 32 Column: 5"))));
});
uuid__000000000001f05d;
});}
#line 14 "PLATFORM/C/LIB/list.lm"
char* head_CL_ArrowStringListString(LM_ListString uuid__000000000001f064){LM_ListString uuid__000000000001f065;
LM_ListString uuid__000000000001f066;
return ({({({uuid__000000000001f065=uuid__000000000001f064;({});})
;
(({({uuid__000000000001f066=uuid__000000000001f065;({});})
;
((uuid__000000000001f066.field_0)==(1));
})?(fail_CL_ArrowNeverString("List::head is fallible\n")):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 15 Column: 5"))));
});
(((LM_ListString)(uuid__000000000001f064)).field_2);
});}
#line 22 "PLATFORM/C/LIB/array.lm"
LM_ListType* close_CL_ArrowArray_QM_ListTypeListType(LM_ListType uuid__000000000001f067){LM_ListType* uuid__000000000001f068;
return ({({({uuid__000000000001f068=((LM_ListType*)((malloc((sizeof(LM_ListType))))));({});})
;
(uuid__000000000001f068[(0)]=uuid__000000000001f067);
});
uuid__000000000001f068;
});}
#line 9 "PLATFORM/C/LIB/list.lm"
LM_ListType list_CL__CL_cons_CL_ArrowListTypeConsListTypeType(LM_Type uuid__000000000001f069,LM_ListType uuid__000000000001f06a){return ({({uuid__000000000001f06a=({LM_ListType rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_ListTypeListType(uuid__000000000001f06a));rvalue.field_2=uuid__000000000001f069;rvalue;});({});});
uuid__000000000001f06a;
});}
#line 22 "PLATFORM/C/LIB/array.lm"
LM_Type* close_CL_ArrowArray_QM_TypeType(LM_Type uuid__000000000001f06e){LM_Type* uuid__000000000001f06f;
return ({({({uuid__000000000001f06f=((LM_Type*)((malloc((sizeof(LM_Type))))));({});})
;
(uuid__000000000001f06f[(0)]=uuid__000000000001f06e);
});
uuid__000000000001f06f;
});}
#line 91 "PLATFORM/C/LIB/list.lm"
LM_Type _DT_nth_CL_ArrowTypeConsTypeConsU64ListType(LM_ListType uuid__000000000001f070,unsigned long uuid__000000000001f071,LM_Type uuid__000000000001f072){LM_ListType uuid__000000000001f073;
LM_ListType uuid__000000000001f074;
char uuid__000000000001f075;
LM_ListType uuid__000000000001f076;
LM_ListType uuid__000000000001f077;
LM_ListType uuid__000000000001f078;
char uuid__000000000001f079;
LM_Type uuid__000000000001f07a;
return ({({({while((uuid__000000000001f071>(0))){((void)(({({({uuid__000000000001f073=uuid__000000000001f070;({});})
;
(({({({({uuid__000000000001f074=uuid__000000000001f073;({});})
;
({uuid__000000000001f075=(0);({});})
;
});
(((uuid__000000000001f074.field_0)==(0))?(({({uuid__000000000001f076=(*(((LM_ListType)(uuid__000000000001f074)).field_1));({});})
;
1;
})?(1?({uuid__000000000001f075=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f075==(1));
})?({uuid__000000000001f070=uuid__000000000001f076;({});}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 93 Column: 8"))));
});
({uuid__000000000001f071=(uuid__000000000001f071-(1));({});});
})));};});
({({uuid__000000000001f077=uuid__000000000001f070;({});})
;
(({({({({uuid__000000000001f078=uuid__000000000001f077;({});})
;
({uuid__000000000001f079=(0);({});})
;
});
(((uuid__000000000001f078.field_0)==(0))?(1?(({({uuid__000000000001f07a=(((LM_ListType)(uuid__000000000001f078)).field_2);({});})
;
1;
})?({uuid__000000000001f079=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f079==(1));
})?({uuid__000000000001f072=uuid__000000000001f07a;({});}):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 100 Column: 5"))));
});
});
uuid__000000000001f072;
});}
#line 31 "PLATFORM/C/LIB/list.lm"
LM_ListType tail_CL_ArrowListTypeListType(LM_ListType uuid__000000000001f07b){LM_ListType uuid__000000000001f07c;
LM_ListType uuid__000000000001f07d;
LM_ListType uuid__000000000001f07e;
char uuid__000000000001f07f;
LM_ListType uuid__000000000001f080;
LM_Type uuid__000000000001f081;
return ({({({uuid__000000000001f07c=uuid__000000000001f07b;({});})
;
(({({uuid__000000000001f07d=uuid__000000000001f07c;({});})
;
((uuid__000000000001f07d.field_0)==(1));
})?(fail_CL_ArrowNeverString("List::head is fallible\n")):(({({({({uuid__000000000001f07e=uuid__000000000001f07c;({});})
;
({uuid__000000000001f07f=(0);({});})
;
});
(((uuid__000000000001f07e.field_0)==(0))?(({({uuid__000000000001f080=(*(((LM_ListType)(uuid__000000000001f07e)).field_1));({});})
;
1;
})?(({({uuid__000000000001f081=(((LM_ListType)(uuid__000000000001f07e)).field_2);({});})
;
1;
})?({uuid__000000000001f07f=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f07f==(1));
})?({uuid__000000000001f07b=uuid__000000000001f080;({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 32 Column: 5"))));
});
uuid__000000000001f07b;
});}
#line 14 "PLATFORM/C/LIB/list.lm"
LM_Type head_CL_ArrowTypeListType(LM_ListType uuid__000000000001f082){LM_ListType uuid__000000000001f083;
LM_ListType uuid__000000000001f084;
return ({({({uuid__000000000001f083=uuid__000000000001f082;({});})
;
(({({uuid__000000000001f084=uuid__000000000001f083;({});})
;
((uuid__000000000001f084.field_0)==(1));
})?(fail_CL_ArrowNeverString("List::head is fallible\n")):(1?({}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 15 Column: 5"))));
});
(((LM_ListType)(uuid__000000000001f082)).field_2);
});}
#line 23 "PLATFORM/C/LIB/list.lm"
unsigned long _DT_has_SB_head_CL_ArrowU64ListType(LM_ListType uuid__000000000001f085){LM_ListType uuid__000000000001f086;
LM_ListType uuid__000000000001f087;
return ({({uuid__000000000001f086=uuid__000000000001f085;({});})
;
(({({uuid__000000000001f087=uuid__000000000001f086;({});})
;
((uuid__000000000001f087.field_0)==(1));
})?(0):(1?(1):({(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 24 Column: 5"));unsigned long rvalue;rvalue;})));
});}
#line 52 "PLATFORM/C/LIB/list.lm"
unsigned long _DT_length_CL_ArrowU64ListType(LM_ListType uuid__000000000001f088){unsigned long uuid__000000000001f089;
LM_ListType uuid__000000000001f08a;
LM_ListType uuid__000000000001f08b;
char uuid__000000000001f08c;
LM_ListType uuid__000000000001f08d;
return ({({({uuid__000000000001f089=(0);({});})
;
({while((non_SB_zero_CL_ArrowU64ListType(uuid__000000000001f088))){((void)(({({({uuid__000000000001f08a=uuid__000000000001f088;({});})
;
(({({({({uuid__000000000001f08b=uuid__000000000001f08a;({});})
;
({uuid__000000000001f08c=(0);({});})
;
});
(((uuid__000000000001f08b.field_0)==(0))?(({({uuid__000000000001f08d=(*(((LM_ListType)(uuid__000000000001f08b)).field_1));({});})
;
1;
})?(1?({uuid__000000000001f08c=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f08c==(1));
})?({uuid__000000000001f088=uuid__000000000001f08d;({});}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 55 Column: 8")));
});
({uuid__000000000001f089=(uuid__000000000001f089+(1));({});});
})));};});
});
uuid__000000000001f089;
});}
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListType(LM_ListType uuid__000000000001f08e){return ((uuid__000000000001f08e.field_0)!=(({LM_ListType rvalue={1};rvalue;}).field_0));}
#line 4 "PLATFORM/C/LIB/list.lm"
LM_ListType cons_CL_ArrowListTypeConsListTypeType(LM_Type uuid__000000000001f090,LM_ListType uuid__000000000001f091){return ({({uuid__000000000001f091=({LM_ListType rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_ListTypeListType(uuid__000000000001f091));rvalue.field_2=uuid__000000000001f090;rvalue;});({});});
uuid__000000000001f091;
});}
#line 5 "PLATFORM/C/LIB/hashtable.lsts"
LM_SmartString _DT_lookup_CL_ArrowSmartStringConsSmartStringConsU64HashtableEqSmartStringU64(LM_HashtableEqSmartStringU64 uuid__000000000001f095,unsigned long uuid__000000000001f096,LM_SmartString uuid__000000000001f097){unsigned long uuid__000000000001f098;
LM_TupleSmartStringU64* uuid__000000000001f099;
unsigned long uuid__000000000001f09a;
unsigned long uuid__000000000001f09b;
LM_TupleSmartStringU64 uuid__000000000001f09c;
return ({({({uuid__000000000001f098=(((LM_HashtableEqSmartStringU64)(uuid__000000000001f095)).field_1002);({});})
;
((uuid__000000000001f098>(0))?({({({({({({({uuid__000000000001f099=(((LM_HashtableEqSmartStringU64)(uuid__000000000001f095)).field_1001);({});})
;
({uuid__000000000001f09a=false_CL_U64;({});})
;
});
({uuid__000000000001f09b=((deep_SB_hash_CL_ArrowU64U64(uuid__000000000001f096))%uuid__000000000001f098);({});})
;
});
({uuid__000000000001f09c=(uuid__000000000001f099[uuid__000000000001f09b]);({});})
;
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleSmartStringU64(uuid__000000000001f09c))&&((uuid__000000000001f09c.field_2)==uuid__000000000001f096))?({uuid__000000000001f09a=true_CL_U64;({});}):({}));
});
({while(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleSmartStringU64(uuid__000000000001f09c))&&(!uuid__000000000001f09a))){((void)(({({({uuid__000000000001f09b=((uuid__000000000001f09b+(1))%uuid__000000000001f098);({});});
({uuid__000000000001f09c=(uuid__000000000001f099[uuid__000000000001f09b]);({});});
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleSmartStringU64(uuid__000000000001f09c))&&((uuid__000000000001f09c.field_2)==uuid__000000000001f096))?({uuid__000000000001f09a=true_CL_U64;({});}):({}));
})));};});
});
(uuid__000000000001f09a?({uuid__000000000001f097=(uuid__000000000001f09c.field_1);({});}):({}));
}):({}));
});
uuid__000000000001f097;
});}
#line 30 "PLATFORM/C/LIB/sized.lm"
unsigned long mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleSmartStringU64(LM_TupleSmartStringU64 uuid__000000000001f0a3){unsigned long uuid__000000000001f0a4;
unsigned long uuid__000000000001f0a5;
return ({({({({uuid__000000000001f0a4=(0);({});})
;
({uuid__000000000001f0a5=(0);({});})
;
});
({while((uuid__000000000001f0a4<(sizeof(LM_TupleSmartStringU64)))){((void)(({(((((char*)((&uuid__000000000001f0a3)))[uuid__000000000001f0a4])!=(0))?({uuid__000000000001f0a5=(1);({});}):({}));
({uuid__000000000001f0a4=(uuid__000000000001f0a4+(1));({});});
})));};});
});
uuid__000000000001f0a5;
});}
#line 22 "PLATFORM/C/LIB/array.lm"
LM_TContext* close_CL_ArrowArray_QM_TContextTContext(LM_TContext uuid__000000000001f0a6){LM_TContext* uuid__000000000001f0a7;
return ({({({uuid__000000000001f0a7=((LM_TContext*)((malloc((sizeof(LM_TContext))))));({});})
;
(uuid__000000000001f0a7[(0)]=uuid__000000000001f0a6);
});
uuid__000000000001f0a7;
});}
#line 40 "PLATFORM/C/LIB/hashtable.lsts"
LM_HashtableEqListTupleTypeTypeTupleU64String _DT_bind_CL_ArrowHashtableEqListTupleTypeTypeTupleU64StringConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0a8,LM_TupleU64String uuid__000000000001f0af,LM_ListTupleTypeType uuid__000000000001f0b0){unsigned long uuid__000000000001f0b1;
unsigned long uuid__000000000001f0b2;
LM_TupleListTupleTypeTypeTupleU64String* uuid__000000000001f0b3;
unsigned long uuid__000000000001f0b4;
unsigned long uuid__000000000001f0b5;
LM_TupleListTupleTypeTypeTupleU64String* uuid__000000000001f0b6;
unsigned long uuid__000000000001f0b7;
LM_TupleListTupleTypeTypeTupleU64String uuid__000000000001f0b8;
unsigned long uuid__000000000001f0b9;
return ({({({({({({({((is_CL_ArrowU64ConsHashtableEqListTupleTypeTypeTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(uuid__000000000001f0a8,({LM_HashtableEqListTupleTypeTypeTupleU64String rvalue={0};rvalue;})))?({uuid__000000000001f0a8=({LM_HashtableEqListTupleTypeTypeTupleU64String rvalue={.field_0=1};rvalue.field_1001=((LM_TupleListTupleTypeTypeTupleU64String*)((0)));rvalue.field_1002=(0);rvalue.field_1003=(0);rvalue;});({});}):({}));
({uuid__000000000001f0b1=(((LM_HashtableEqListTupleTypeTypeTupleU64String)(uuid__000000000001f0a8)).field_1003);({});})
;
});
({uuid__000000000001f0b2=(((LM_HashtableEqListTupleTypeTypeTupleU64String)(uuid__000000000001f0a8)).field_1002);({});})
;
});
({uuid__000000000001f0b3=(((LM_HashtableEqListTupleTypeTypeTupleU64String)(uuid__000000000001f0a8)).field_1001);({});})
;
});
(((uuid__000000000001f0b1*(3))>=uuid__000000000001f0b2)?({({({({({({({({uuid__000000000001f0b4=(max_CL_ArrowU64ConsU64U64((1000),(uuid__000000000001f0b1*(30))));({});})
;
({uuid__000000000001f0b5=((sizeof(LM_TupleListTupleTypeTypeTupleU64String))*uuid__000000000001f0b4);({});})
;
});
({uuid__000000000001f0b6=((LM_TupleListTupleTypeTypeTupleU64String*)((malloc(uuid__000000000001f0b5))));({});})
;
});
(memset(uuid__000000000001f0b6,(0),uuid__000000000001f0b5));
});
({uuid__000000000001f0b7=(0);({});})
;
});
({while((uuid__000000000001f0b7<uuid__000000000001f0b2)){((void)(({({({uuid__000000000001f0b8=(uuid__000000000001f0b3[uuid__000000000001f0b7]);({});})
;
((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0b8))?(_DT_bind_SB_eq_CL_ArrowU64ConsListTupleTypeTypeConsTupleU64StringConsU64Array_QM_TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0b6,uuid__000000000001f0b4,(uuid__000000000001f0b8.field_2),(uuid__000000000001f0b8.field_1))):({}));
});
({uuid__000000000001f0b7=(uuid__000000000001f0b7+(1));({});});
})));};});
});
({uuid__000000000001f0b3=uuid__000000000001f0b6;({});});
});
({uuid__000000000001f0b2=uuid__000000000001f0b4;({});});
}):({}));
});
({uuid__000000000001f0b9=(_DT_bind_SB_eq_CL_ArrowU64ConsListTupleTypeTypeConsTupleU64StringConsU64Array_QM_TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0b3,uuid__000000000001f0b2,uuid__000000000001f0af,uuid__000000000001f0b0));({});})
;
});
((!uuid__000000000001f0b9)?({uuid__000000000001f0b1=(uuid__000000000001f0b1+(1));({});}):({}));
});
({LM_HashtableEqListTupleTypeTypeTupleU64String rvalue={.field_0=1};rvalue.field_1001=uuid__000000000001f0b3;rvalue.field_1002=uuid__000000000001f0b2;rvalue.field_1003=uuid__000000000001f0b1;rvalue;});
});}
#line 68 "PLATFORM/C/LIB/hashtable.lsts"
unsigned long _DT_bind_SB_eq_CL_ArrowU64ConsListTupleTypeTypeConsTupleU64StringConsU64Array_QM_TupleListTupleTypeTypeTupleU64String(LM_TupleListTupleTypeTypeTupleU64String* uuid__000000000001f0ba,unsigned long uuid__000000000001f0bb,LM_TupleU64String uuid__000000000001f0c2,LM_ListTupleTypeType uuid__000000000001f0c3){unsigned long uuid__000000000001f0c4;
unsigned long uuid__000000000001f0c5;
LM_TupleListTupleTypeTypeTupleU64String uuid__000000000001f0c6;
return ({({({({({({({uuid__000000000001f0c4=false_CL_U64;({});})
;
({uuid__000000000001f0c5=((deep_SB_hash_CL_ArrowU64TupleU64String(uuid__000000000001f0c2))%uuid__000000000001f0bb);({});})
;
});
({uuid__000000000001f0c6=(uuid__000000000001f0ba[uuid__000000000001f0c5]);({});})
;
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0c6))&&(_EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String((uuid__000000000001f0c6.field_2),uuid__000000000001f0c2)))?({uuid__000000000001f0c4=true_CL_U64;({});}):({}));
});
({while(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0c6))&&(!uuid__000000000001f0c4))){((void)(({({({uuid__000000000001f0c5=((uuid__000000000001f0c5+(1))%uuid__000000000001f0bb);({});});
({uuid__000000000001f0c6=(uuid__000000000001f0ba[uuid__000000000001f0c5]);({});});
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0c6))&&(_EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String((uuid__000000000001f0c6.field_2),uuid__000000000001f0c2)))?({uuid__000000000001f0c4=true_CL_U64;({});}):({}));
})));};});
});
(uuid__000000000001f0ba[uuid__000000000001f0c5]=({LM_TupleListTupleTypeTypeTupleU64String rvalue={.field_0=0};rvalue.field_1=uuid__000000000001f0c3;rvalue.field_2=uuid__000000000001f0c2;rvalue;}));
});
uuid__000000000001f0c4;
});}
#line 30 "PLATFORM/C/LIB/sized.lm"
unsigned long mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(LM_TupleListTupleTypeTypeTupleU64String uuid__000000000001f0c7){unsigned long uuid__000000000001f0c8;
unsigned long uuid__000000000001f0c9;
return ({({({({uuid__000000000001f0c8=(0);({});})
;
({uuid__000000000001f0c9=(0);({});})
;
});
({while((uuid__000000000001f0c8<(sizeof(LM_TupleListTupleTypeTypeTupleU64String)))){((void)(({(((((char*)((&uuid__000000000001f0c7)))[uuid__000000000001f0c8])!=(0))?({uuid__000000000001f0c9=(1);({});}):({}));
({uuid__000000000001f0c8=(uuid__000000000001f0c8+(1));({});});
})));};});
});
uuid__000000000001f0c9;
});}
#line 16 "PLATFORM/C/LIB/tuple.lsts"
unsigned long _EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String(LM_TupleU64String uuid__000000000001f0d0,LM_TupleU64String uuid__000000000001f0d7){return ((_EQ__EQ__CL_ArrowU64ConsStringString((uuid__000000000001f0d0.field_2),(uuid__000000000001f0d7.field_2)))&&((uuid__000000000001f0d0.field_1)==(uuid__000000000001f0d7.field_1)));}
#line 8 "PLATFORM/C/LIB/tuple.lsts"
unsigned long deep_SB_hash_CL_ArrowU64TupleU64String(LM_TupleU64String uuid__000000000001f0de){return ((deep_SB_hash_CL_ArrowU64String((uuid__000000000001f0de.field_2)))+(deep_SB_hash_CL_ArrowU64U64((uuid__000000000001f0de.field_1))));}
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsHashtableEqListTupleTypeTypeTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0df,LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0e0){unsigned long uuid__000000000001f0e1;
unsigned long uuid__000000000001f0e2;
char uuid__000000000001f0e3;
char uuid__000000000001f0e4;
return ({({({({uuid__000000000001f0e1=(0);({});})
;
({uuid__000000000001f0e2=(1);({});})
;
});
({while((uuid__000000000001f0e1<(sizeof(LM_HashtableEqListTupleTypeTypeTupleU64String)))){((void)(({({({({uuid__000000000001f0e3=(((char*)((&uuid__000000000001f0df)))[uuid__000000000001f0e1]);({});})
;
({uuid__000000000001f0e4=(((char*)((&uuid__000000000001f0e0)))[uuid__000000000001f0e1]);({});})
;
});
((uuid__000000000001f0e3==uuid__000000000001f0e4)?({}):({uuid__000000000001f0e2=(0);({});}));
});
({uuid__000000000001f0e1=(uuid__000000000001f0e1+(1));({});});
})));};});
});
uuid__000000000001f0e2;
});}
#line 4 "PLATFORM/C/LIB/list.lm"
LM_ListTupleTypeType cons_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeTupleTypeType(LM_TupleTypeType uuid__000000000001f0eb,LM_ListTupleTypeType uuid__000000000001f0ec){return ({({uuid__000000000001f0ec=({LM_ListTupleTypeType rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_ListTupleTypeTypeListTupleTypeType(uuid__000000000001f0ec));rvalue.field_2=uuid__000000000001f0eb;rvalue;});({});});
uuid__000000000001f0ec;
});}
#line 22 "PLATFORM/C/LIB/array.lm"
LM_ListTupleTypeType* close_CL_ArrowArray_QM_ListTupleTypeTypeListTupleTypeType(LM_ListTupleTypeType uuid__000000000001f0ed){LM_ListTupleTypeType* uuid__000000000001f0ee;
return ({({({uuid__000000000001f0ee=((LM_ListTupleTypeType*)((malloc((sizeof(LM_ListTupleTypeType))))));({});})
;
(uuid__000000000001f0ee[(0)]=uuid__000000000001f0ed);
});
uuid__000000000001f0ee;
});}
#line 5 "PLATFORM/C/LIB/hashtable.lsts"
LM_ListTupleTypeType _DT_lookup_CL_ArrowListTupleTypeTypeConsListTupleTypeTypeConsTupleU64StringHashtableEqListTupleTypeTypeTupleU64String(LM_HashtableEqListTupleTypeTypeTupleU64String uuid__000000000001f0ef,LM_TupleU64String uuid__000000000001f0f6,LM_ListTupleTypeType uuid__000000000001f0f7){unsigned long uuid__000000000001f0f8;
LM_TupleListTupleTypeTypeTupleU64String* uuid__000000000001f0f9;
unsigned long uuid__000000000001f0fa;
unsigned long uuid__000000000001f0fb;
LM_TupleListTupleTypeTypeTupleU64String uuid__000000000001f0fc;
return ({({({uuid__000000000001f0f8=(((LM_HashtableEqListTupleTypeTypeTupleU64String)(uuid__000000000001f0ef)).field_1002);({});})
;
((uuid__000000000001f0f8>(0))?({({({({({({({uuid__000000000001f0f9=(((LM_HashtableEqListTupleTypeTypeTupleU64String)(uuid__000000000001f0ef)).field_1001);({});})
;
({uuid__000000000001f0fa=false_CL_U64;({});})
;
});
({uuid__000000000001f0fb=((deep_SB_hash_CL_ArrowU64TupleU64String(uuid__000000000001f0f6))%uuid__000000000001f0f8);({});})
;
});
({uuid__000000000001f0fc=(uuid__000000000001f0f9[uuid__000000000001f0fb]);({});})
;
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0fc))&&(_EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String((uuid__000000000001f0fc.field_2),uuid__000000000001f0f6)))?({uuid__000000000001f0fa=true_CL_U64;({});}):({}));
});
({while(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0fc))&&(!uuid__000000000001f0fa))){((void)(({({({uuid__000000000001f0fb=((uuid__000000000001f0fb+(1))%uuid__000000000001f0f8);({});});
({uuid__000000000001f0fc=(uuid__000000000001f0f9[uuid__000000000001f0fb]);({});});
});
(((mem_SB_is_SB_non_SB_zero_CL_ArrowU64TupleListTupleTypeTypeTupleU64String(uuid__000000000001f0fc))&&(_EQ__EQ__CL_ArrowU64ConsTupleU64StringTupleU64String((uuid__000000000001f0fc.field_2),uuid__000000000001f0f6)))?({uuid__000000000001f0fa=true_CL_U64;({});}):({}));
})));};});
});
(uuid__000000000001f0fa?({uuid__000000000001f0f7=(uuid__000000000001f0fc.field_1);({});}):({}));
}):({}));
});
uuid__000000000001f0f7;
});}
#line 18 "PLATFORM/C/LIB/sized.lm"
unsigned long is_CL_ArrowU64ConsTypeType(LM_Type uuid__000000000001f0fd,LM_Type uuid__000000000001f0fe){unsigned long uuid__000000000001f0ff;
unsigned long uuid__000000000001f100;
char uuid__000000000001f101;
char uuid__000000000001f102;
return ({({({({uuid__000000000001f0ff=(0);({});})
;
({uuid__000000000001f100=(1);({});})
;
});
({while((uuid__000000000001f0ff<(sizeof(LM_Type)))){((void)(({({({({uuid__000000000001f101=(((char*)((&uuid__000000000001f0fd)))[uuid__000000000001f0ff]);({});})
;
({uuid__000000000001f102=(((char*)((&uuid__000000000001f0fe)))[uuid__000000000001f0ff]);({});})
;
});
((uuid__000000000001f101==uuid__000000000001f102)?({}):({uuid__000000000001f100=(0);({});}));
});
({uuid__000000000001f0ff=(uuid__000000000001f0ff+(1));({});});
})));};});
});
uuid__000000000001f100;
});}
#line 2 "PLATFORM/C/LIB/list.lm"
unsigned long non_SB_zero_CL_ArrowU64ListTupleTypeType(LM_ListTupleTypeType uuid__000000000001f103){return ((uuid__000000000001f103.field_0)!=(({LM_ListTupleTypeType rvalue={1};rvalue;}).field_0));}
#line 64 "PLATFORM/C/LIB/list.lm"
LM_ListType _DT_reverse_CL_ArrowListTypeListType(LM_ListType uuid__000000000001f104){LM_ListType uuid__000000000001f105;
LM_ListType uuid__000000000001f107;
LM_ListType uuid__000000000001f108;
char uuid__000000000001f109;
LM_ListType uuid__000000000001f10a;
LM_Type uuid__000000000001f10b;
return ({({({uuid__000000000001f105=({LM_ListType rvalue={1};rvalue;});({});})
;
({while((non_SB_zero_CL_ArrowU64ListType(uuid__000000000001f104))){((void)(({({uuid__000000000001f107=uuid__000000000001f104;({});})
;
(({({({({uuid__000000000001f108=uuid__000000000001f107;({});})
;
({uuid__000000000001f109=(0);({});})
;
});
(((uuid__000000000001f108.field_0)==(0))?(({({uuid__000000000001f10a=(*(((LM_ListType)(uuid__000000000001f108)).field_1));({});})
;
1;
})?(({({uuid__000000000001f10b=(((LM_ListType)(uuid__000000000001f108)).field_2);({});})
;
1;
})?({uuid__000000000001f109=(1);({});}):({})):({})):({}));
});
(uuid__000000000001f109==(1));
})?({({uuid__000000000001f105=({LM_ListType rvalue={.field_0=0};rvalue.field_1=(close_CL_ArrowArray_QM_ListTypeListType(uuid__000000000001f105));rvalue.field_2=uuid__000000000001f10b;rvalue;});({});});
({uuid__000000000001f104=uuid__000000000001f10a;({});});
}):(fail_CL_ArrowNeverConsStringString("Pattern Match Failure","File: PLATFORM/C/LIB/list.lm Line: 66 Column: 25")));
})));};});
});
uuid__000000000001f105;
});}
int main(){
uuid__000000000001eb7c=(intern_CL_ArrowSmartStringString("()"));
uuid__000000000001eb85=(intern_CL_ArrowSmartStringString("("));
uuid__000000000001eb87=(intern_CL_ArrowSmartStringString(" "));
uuid__000000000001eb89=(intern_CL_ArrowSmartStringString(")"));
uuid__000000000001eb8e=(intern_CL_ArrowSmartStringString("&["));
uuid__000000000001eb90=(intern_CL_ArrowSmartStringString("]"));
uuid__000000000001ed14=(intern_CL_ArrowSmartStringString("[Unknown File]"));
true_CL_U64=(1);
false_CL_U64=(0);
minimum_SB_I64_CL_I64=(-2147483648);
uuid_SB_counter_CL_U64=(0);
smart_SB_token_SB_path_SB_index_CL_HashtableEqSmartStringU64=({LM_HashtableEqSmartStringU64 rvalue={0};rvalue;});
quick_SB_prop_CL_HashtableEqListTupleTypeTypeTupleU64String=({LM_HashtableEqListTupleTypeTypeTupleU64String rvalue={0};rvalue;});
t_SB_A_CL_Type=(t1_CL_ArrowTypeString("A"));
t_SB_B_CL_Type=(t2_CL_ArrowTypeConsTypeString("B",t_SB_A_CL_Type));
t_SB_C_CL_Type=(t1_CL_ArrowTypeString("C"));
(add_SB_quick_SB_prop_CL_ArrowNilConsTypeConsTypeType(t_SB_A_CL_Type,({LM_Type rvalue={3};rvalue;}),t_SB_C_CL_Type));
ti1_CL_Type=(enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(t_SB_A_CL_Type,t_SB_A_CL_Type));
ti2_CL_Type=(enrich_SB_quick_SB_prop_CL_ArrowTypeConsTypeType(ti1_CL_Type,ti1_CL_Type));
(print_CL_ArrowNilType(ti2_CL_Type));
return 0;
}
