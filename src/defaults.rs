// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::args::Macro;

macro_rules! defined {
    ( $lhs: expr, $rhs: expr) => {{
        Macro::Defined(($lhs.to_string(), $rhs.to_string()))
    }};
}

pub fn get_defined() -> Vec<Macro> {
    // clang++-9 -dM -E - </dev/null
    vec![
        defined!("__has_attribute(x)", "0"),
        defined!("__has_builtin(x)", "0"),
        defined!("__has_extension(x)", "0"),
        defined!("__has_feature(x)", "0"),
        defined!("__has_include(x)", "0"),
        defined!("__has_include_next(x)", "0"),
        defined!("__has_warning(x)", "0"),
        defined!("_LP64", "1"),
        defined!("__ATOMIC_ACQUIRE", "2"),
        defined!("__ATOMIC_ACQ_REL", "4"),
        defined!("__ATOMIC_CONSUME", "1"),
        defined!("__ATOMIC_RELAXED", "0"),
        defined!("__ATOMIC_RELEASE", "3"),
        defined!("__ATOMIC_SEQ_CST", "5"),
        defined!("__BIGGEST_ALIGNMENT__", "16"),
        defined!("__BYTE_ORDER__", "__ORDER_LITTLE_ENDIAN__"),
        defined!("__CHAR16_TYPE__", "unsigned short"),
        defined!("__CHAR32_TYPE__", "unsigned int"),
        defined!("__CHAR_BIT__", "8"),
        defined!("__CLANG_ATOMIC_BOOL_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_CHAR16_T_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_CHAR32_T_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_CHAR_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_INT_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_LLONG_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_LONG_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_POINTER_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_SHORT_LOCK_FREE", "2"),
        defined!("__CLANG_ATOMIC_WCHAR_T_LOCK_FREE", "2"),
        defined!("__CONSTANT_CFSTRINGS__", "1"),
        defined!("__DBL_DECIMAL_DIG__", "17"),
        defined!("__DBL_DENORM_MIN__", "4.9406564584124654e-324"),
        defined!("__DBL_DIG__", "15"),
        defined!("__DBL_EPSILON__", "2.2204460492503131e-16"),
        defined!("__DBL_HAS_DENORM__", "1"),
        defined!("__DBL_HAS_INFINITY__", "1"),
        defined!("__DBL_HAS_QUIET_NAN__", "1"),
        defined!("__DBL_MANT_DIG__", "53"),
        defined!("__DBL_MAX_10_EXP__", "308"),
        defined!("__DBL_MAX_EXP__", "1024"),
        defined!("__DBL_MAX__", "1.7976931348623157e+308"),
        defined!("__DBL_MIN_10_EXP__", "(-307)"),
        defined!("__DBL_MIN_EXP__", "(-1021)"),
        defined!("__DBL_MIN__", "2.2250738585072014e-308"),
        defined!("__DECIMAL_DIG__", "__LDBL_DECIMAL_DIG__"),
        defined!("__ELF__", "1"),
        defined!("__FINITE_MATH_ONLY__", "0"),
        defined!("__FLOAT128__", "1"),
        defined!("__FLT_DECIMAL_DIG__", "9"),
        defined!("__FLT_DENORM_MIN__", "1.40129846e-45F"),
        defined!("__FLT_DIG__", "6"),
        defined!("__FLT_EPSILON__", "1.19209290e-7F"),
        defined!("__FLT_EVAL_METHOD__", "0"),
        defined!("__FLT_HAS_DENORM__", "1"),
        defined!("__FLT_HAS_INFINITY__", "1"),
        defined!("__FLT_HAS_QUIET_NAN__", "1"),
        defined!("__FLT_MANT_DIG__", "24"),
        defined!("__FLT_MAX_10_EXP__", "38"),
        defined!("__FLT_MAX_EXP__", "128"),
        defined!("__FLT_MAX__", "3.40282347e+38F"),
        defined!("__FLT_MIN_10_EXP__", "(-37)"),
        defined!("__FLT_MIN_EXP__", "(-125)"),
        defined!("__FLT_MIN__", "1.17549435e-38F"),
        defined!("__FLT_RADIX__", "2"),
        defined!("__FXSR__", "1"),
        defined!("__GCC_ASM_FLAG_OUTPUTS__", "1"),
        defined!("__GCC_ATOMIC_BOOL_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_CHAR16_T_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_CHAR32_T_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_CHAR_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_INT_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_LLONG_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_LONG_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_POINTER_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_SHORT_LOCK_FREE", "2"),
        defined!("__GCC_ATOMIC_TEST_AND_SET_TRUEVAL", "1"),
        defined!("__GCC_ATOMIC_WCHAR_T_LOCK_FREE", "2"),
        defined!("__GCC_HAVE_SYNC_COMPARE_AND_SWAP_1", "1"),
        defined!("__GCC_HAVE_SYNC_COMPARE_AND_SWAP_2", "1"),
        defined!("__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4", "1"),
        defined!("__GCC_HAVE_SYNC_COMPARE_AND_SWAP_8", "1"),
        defined!("__GNUC_MINOR__", "2"),
        defined!("__GNUC_PATCHLEVEL__", "1"),
        defined!("__GNUC_STDC_INLINE__", "1"),
        defined!("__GNUC__", "4"),
        defined!("__GXX_ABI_VERSION", "1002"),
        defined!("__INT16_C_SUFFIX__", ""),
        defined!("__INT16_FMTd__", "\"hd\""),
        defined!("__INT16_FMTi__", "\"hi\""),
        defined!("__INT16_MAX__", "32767"),
        defined!("__INT16_TYPE__", "short"),
        defined!("__INT32_C_SUFFIX__", ""),
        defined!("__INT32_FMTd__", "\"d\""),
        defined!("__INT32_FMTi__", "\"i\""),
        defined!("__INT32_MAX__", "2147483647"),
        defined!("__INT32_TYPE__", "int"),
        defined!("__INT64_C_SUFFIX__", "L"),
        defined!("__INT64_FMTd__", "\"ld\""),
        defined!("__INT64_FMTi__", "\"li\""),
        defined!("__INT64_MAX__", "9223372036854775807L"),
        defined!("__INT64_TYPE__", "long int"),
        defined!("__INT8_C_SUFFIX__", ""),
        defined!("__INT8_FMTd__", "\"hhd\""),
        defined!("__INT8_FMTi__", "\"hhi\""),
        defined!("__INT8_MAX__", "127"),
        defined!("__INT8_TYPE__", "signed char"),
        defined!("__INTMAX_C_SUFFIX__", "L"),
        defined!("__INTMAX_FMTd__", "\"ld\""),
        defined!("__INTMAX_FMTi__", "\"li\""),
        defined!("__INTMAX_MAX__", "9223372036854775807L"),
        defined!("__INTMAX_TYPE__", "long int"),
        defined!("__INTMAX_WIDTH__", "64"),
        defined!("__INTPTR_FMTd__", "\"ld\""),
        defined!("__INTPTR_FMTi__", "\"li\""),
        defined!("__INTPTR_MAX__", "9223372036854775807L"),
        defined!("__INTPTR_TYPE__", "long int"),
        defined!("__INTPTR_WIDTH__", "64"),
        defined!("__INT_FAST16_FMTd__", "\"hd\""),
        defined!("__INT_FAST16_FMTi__", "\"hi\""),
        defined!("__INT_FAST16_MAX__", "32767"),
        defined!("__INT_FAST16_TYPE__", "short"),
        defined!("__INT_FAST32_FMTd__", "\"d\""),
        defined!("__INT_FAST32_FMTi__", "\"i\""),
        defined!("__INT_FAST32_MAX__", "2147483647"),
        defined!("__INT_FAST32_TYPE__", "int"),
        defined!("__INT_FAST64_FMTd__", "\"ld\""),
        defined!("__INT_FAST64_FMTi__", "\"li\""),
        defined!("__INT_FAST64_MAX__", "9223372036854775807L"),
        defined!("__INT_FAST64_TYPE__", "long int"),
        defined!("__INT_FAST8_FMTd__", "\"hhd\""),
        defined!("__INT_FAST8_FMTi__", "\"hhi\""),
        defined!("__INT_FAST8_MAX__", "127"),
        defined!("__INT_FAST8_TYPE__", "signed char"),
        defined!("__INT_LEAST16_FMTd__", "\"hd\""),
        defined!("__INT_LEAST16_FMTi__", "\"hi\""),
        defined!("__INT_LEAST16_MAX__", "32767"),
        defined!("__INT_LEAST16_TYPE__", "short"),
        defined!("__INT_LEAST32_FMTd__", "\"d\""),
        defined!("__INT_LEAST32_FMTi__", "\"i\""),
        defined!("__INT_LEAST32_MAX__", "2147483647"),
        defined!("__INT_LEAST32_TYPE__", "int"),
        defined!("__INT_LEAST64_FMTd__", "\"ld\""),
        defined!("__INT_LEAST64_FMTi__", "\"li\""),
        defined!("__INT_LEAST64_MAX__", "9223372036854775807L"),
        defined!("__INT_LEAST64_TYPE__", "long int"),
        defined!("__INT_LEAST8_FMTd__", "\"hhd\""),
        defined!("__INT_LEAST8_FMTi__", "\"hhi\""),
        defined!("__INT_LEAST8_MAX__", "127"),
        defined!("__INT_LEAST8_TYPE__", "signed char"),
        defined!("__INT_MAX__", "2147483647"),
        defined!("__LDBL_DECIMAL_DIG__", "21"),
        defined!("__LDBL_DENORM_MIN__", "3.64519953188247460253e-4951L"),
        defined!("__LDBL_DIG__", "18"),
        defined!("__LDBL_EPSILON__", "1.08420217248550443401e-19L"),
        defined!("__LDBL_HAS_DENORM__", "1"),
        defined!("__LDBL_HAS_INFINITY__", "1"),
        defined!("__LDBL_HAS_QUIET_NAN__", "1"),
        defined!("__LDBL_MANT_DIG__", "64"),
        defined!("__LDBL_MAX_10_EXP__", "4932"),
        defined!("__LDBL_MAX_EXP__", "16384"),
        defined!("__LDBL_MAX__", "1.18973149535723176502e+4932L"),
        defined!("__LDBL_MIN_10_EXP__", "(-4931)"),
        defined!("__LDBL_MIN_EXP__", "(-16381)"),
        defined!("__LDBL_MIN__", "3.36210314311209350626e-4932L"),
        defined!("__LITTLE_ENDIAN__", "1"),
        defined!("__LONG_LONG_MAX__", "9223372036854775807LL"),
        defined!("__LONG_MAX__", "9223372036854775807L"),
        defined!("__LP64__", "1"),
        defined!("__MMX__", "1"),
        defined!("__NO_INLINE__", "1"),
        defined!("__NO_MATH_INLINES", "1"),
        defined!("__OBJC_BOOL_IS_BOOL", "0"),
        defined!("__OPENCL_MEMORY_SCOPE_ALL_SVM_DEVICES", "3"),
        defined!("__OPENCL_MEMORY_SCOPE_DEVICE", "2"),
        defined!("__OPENCL_MEMORY_SCOPE_SUB_GROUP", "4"),
        defined!("__OPENCL_MEMORY_SCOPE_WORK_GROUP", "1"),
        defined!("__OPENCL_MEMORY_SCOPE_WORK_ITEM", "0"),
        defined!("__ORDER_BIG_ENDIAN__", "4321"),
        defined!("__ORDER_LITTLE_ENDIAN__", "1234"),
        defined!("__ORDER_PDP_ENDIAN__", "3412"),
        defined!("__POINTER_WIDTH__", "64"),
        defined!("__PRAGMA_REDEFINE_EXTNAME", "1"),
        defined!("__PTRDIFF_FMTd__", "\"ld\""),
        defined!("__PTRDIFF_FMTi__", "\"li\""),
        defined!("__PTRDIFF_MAX__", "9223372036854775807L"),
        defined!("__PTRDIFF_TYPE__", "long int"),
        defined!("__PTRDIFF_WIDTH__", "64"),
        defined!("__REGISTER_PREFIX__", ""),
        defined!("__SCHAR_MAX__", "127"),
        defined!("__SEG_FS", "1"),
        defined!("__SEG_GS", "1"),
        defined!("__SHRT_MAX__", "32767"),
        defined!("__SIG_ATOMIC_MAX__", "2147483647"),
        defined!("__SIG_ATOMIC_WIDTH__", "32"),
        defined!("__SIZEOF_DOUBLE__", "8"),
        defined!("__SIZEOF_FLOAT128__", "16"),
        defined!("__SIZEOF_FLOAT__", "4"),
        defined!("__SIZEOF_INT128__", "16"),
        defined!("__SIZEOF_INT__", "4"),
        defined!("__SIZEOF_LONG_DOUBLE__", "16"),
        defined!("__SIZEOF_LONG_LONG__", "8"),
        defined!("__SIZEOF_LONG__", "8"),
        defined!("__SIZEOF_POINTER__", "8"),
        defined!("__SIZEOF_PTRDIFF_T__", "8"),
        defined!("__SIZEOF_SHORT__", "2"),
        defined!("__SIZEOF_SIZE_T__", "8"),
        defined!("__SIZEOF_WCHAR_T__", "4"),
        defined!("__SIZEOF_WINT_T__", "4"),
        defined!("__SIZE_FMTX__", "\"lX\""),
        defined!("__SIZE_FMTo__", "\"lo\""),
        defined!("__SIZE_FMTu__", "\"lu\""),
        defined!("__SIZE_FMTx__", "\"lx\""),
        defined!("__SIZE_MAX__", "18446744073709551615UL"),
        defined!("__SIZE_TYPE__", "long unsigned int"),
        defined!("__SIZE_WIDTH__", "64"),
        defined!("__SSE2_MATH__", "1"),
        defined!("__SSE2__", "1"),
        defined!("__SSE_MATH__", "1"),
        defined!("__SSE__", "1"),
        defined!("__STDC_HOSTED__", "1"),
        defined!("__STDC_UTF_16__", "1"),
        defined!("__STDC_UTF_32__", "1"),
        defined!("__STDC_VERSION__", "201112L"),
        defined!("__STDC__", "1"),
        defined!("__UINT16_C_SUFFIX__", ""),
        defined!("__UINT16_FMTX__", "\"hX\""),
        defined!("__UINT16_FMTo__", "\"ho\""),
        defined!("__UINT16_FMTu__", "\"hu\""),
        defined!("__UINT16_FMTx__", "\"hx\""),
        defined!("__UINT16_MAX__", "65535"),
        defined!("__UINT16_TYPE__", "unsigned short"),
        defined!("__UINT32_C_SUFFIX__", "U"),
        defined!("__UINT32_FMTX__", "\"X\""),
        defined!("__UINT32_FMTo__", "\"o\""),
        defined!("__UINT32_FMTu__", "\"u\""),
        defined!("__UINT32_FMTx__", "\"x\""),
        defined!("__UINT32_MAX__", "4294967295U"),
        defined!("__UINT32_TYPE__", "unsigned int"),
        defined!("__UINT64_C_SUFFIX__", "UL"),
        defined!("__UINT64_FMTX__", "\"lX\""),
        defined!("__UINT64_FMTo__", "\"lo\""),
        defined!("__UINT64_FMTu__", "\"lu\""),
        defined!("__UINT64_FMTx__", "\"lx\""),
        defined!("__UINT64_MAX__", "18446744073709551615UL"),
        defined!("__UINT64_TYPE__", "long unsigned int"),
        defined!("__UINT8_C_SUFFIX__", ""),
        defined!("__UINT8_FMTX__", "\"hhX\""),
        defined!("__UINT8_FMTo__", "\"hho\""),
        defined!("__UINT8_FMTu__", "\"hhu\""),
        defined!("__UINT8_FMTx__", "\"hhx\""),
        defined!("__UINT8_MAX__", "255"),
        defined!("__UINT8_TYPE__", "unsigned char"),
        defined!("__UINTMAX_C_SUFFIX__", "UL"),
        defined!("__UINTMAX_FMTX__", "\"lX\""),
        defined!("__UINTMAX_FMTo__", "\"lo\""),
        defined!("__UINTMAX_FMTu__", "\"lu\""),
        defined!("__UINTMAX_FMTx__", "\"lx\""),
        defined!("__UINTMAX_MAX__", "18446744073709551615UL"),
        defined!("__UINTMAX_TYPE__", "long unsigned int"),
        defined!("__UINTMAX_WIDTH__", "64"),
        defined!("__UINTPTR_FMTX__", "\"lX\""),
        defined!("__UINTPTR_FMTo__", "\"lo\""),
        defined!("__UINTPTR_FMTu__", "\"lu\""),
        defined!("__UINTPTR_FMTx__", "\"lx\""),
        defined!("__UINTPTR_MAX__", "18446744073709551615UL"),
        defined!("__UINTPTR_TYPE__", "long unsigned int"),
        defined!("__UINTPTR_WIDTH__", "64"),
        defined!("__UINT_FAST16_FMTX__", "\"hX\""),
        defined!("__UINT_FAST16_FMTo__", "\"ho\""),
        defined!("__UINT_FAST16_FMTu__", "\"hu\""),
        defined!("__UINT_FAST16_FMTx__", "\"hx\""),
        defined!("__UINT_FAST16_MAX__", "65535"),
        defined!("__UINT_FAST16_TYPE__", "unsigned short"),
        defined!("__UINT_FAST32_FMTX__", "\"X\""),
        defined!("__UINT_FAST32_FMTo__", "\"o\""),
        defined!("__UINT_FAST32_FMTu__", "\"u\""),
        defined!("__UINT_FAST32_FMTx__", "\"x\""),
        defined!("__UINT_FAST32_MAX__", "4294967295U"),
        defined!("__UINT_FAST32_TYPE__", "unsigned int"),
        defined!("__UINT_FAST64_FMTX__", "\"lX\""),
        defined!("__UINT_FAST64_FMTo__", "\"lo\""),
        defined!("__UINT_FAST64_FMTu__", "\"lu\""),
        defined!("__UINT_FAST64_FMTx__", "\"lx\""),
        defined!("__UINT_FAST64_MAX__", "18446744073709551615UL"),
        defined!("__UINT_FAST64_TYPE__", "long unsigned int"),
        defined!("__UINT_FAST8_FMTX__", "\"hhX\""),
        defined!("__UINT_FAST8_FMTo__", "\"hho\""),
        defined!("__UINT_FAST8_FMTu__", "\"hhu\""),
        defined!("__UINT_FAST8_FMTx__", "\"hhx\""),
        defined!("__UINT_FAST8_MAX__", "255"),
        defined!("__UINT_FAST8_TYPE__", "unsigned char"),
        defined!("__UINT_LEAST16_FMTX__", "\"hX\""),
        defined!("__UINT_LEAST16_FMTo__", "\"ho\""),
        defined!("__UINT_LEAST16_FMTu__", "\"hu\""),
        defined!("__UINT_LEAST16_FMTx__", "\"hx\""),
        defined!("__UINT_LEAST16_MAX__", "65535"),
        defined!("__UINT_LEAST16_TYPE__", "unsigned short"),
        defined!("__UINT_LEAST32_FMTX__", "\"X\""),
        defined!("__UINT_LEAST32_FMTo__", "\"o\""),
        defined!("__UINT_LEAST32_FMTu__", "\"u\""),
        defined!("__UINT_LEAST32_FMTx__", "\"x\""),
        defined!("__UINT_LEAST32_MAX__", "4294967295U"),
        defined!("__UINT_LEAST32_TYPE__", "unsigned int"),
        defined!("__UINT_LEAST64_FMTX__", "\"lX\""),
        defined!("__UINT_LEAST64_FMTo__", "\"lo\""),
        defined!("__UINT_LEAST64_FMTu__", "\"lu\""),
        defined!("__UINT_LEAST64_FMTx__", "\"lx\""),
        defined!("__UINT_LEAST64_MAX__", "18446744073709551615UL"),
        defined!("__UINT_LEAST64_TYPE__", "long unsigned int"),
        defined!("__UINT_LEAST8_FMTX__", "\"hhX\""),
        defined!("__UINT_LEAST8_FMTo__", "\"hho\""),
        defined!("__UINT_LEAST8_FMTu__", "\"hhu\""),
        defined!("__UINT_LEAST8_FMTx__", "\"hhx\""),
        defined!("__UINT_LEAST8_MAX__", "255"),
        defined!("__UINT_LEAST8_TYPE__", "unsigned char"),
        defined!("__USER_LABEL_PREFIX__", ""),
        defined!("__VERSION__", "\"Clang 9.0.0 \""),
        defined!("__WCHAR_MAX__", "2147483647"),
        defined!("__WCHAR_TYPE__", "int"),
        defined!("__WCHAR_WIDTH__", "32"),
        defined!("__WINT_MAX__", "4294967295U"),
        defined!("__WINT_TYPE__", "unsigned int"),
        defined!("__WINT_UNSIGNED__", "1"),
        defined!("__WINT_WIDTH__", "32"),
        defined!("__amd64", "1"),
        defined!("__amd64__", "1"),
        defined!("__clang__", "1"),
        defined!("__clang_major__", "9"),
        defined!("__clang_minor__", "0"),
        defined!("__clang_patchlevel__", "0"),
        defined!("__clang_version__", "\"9.0.0 \""),
        defined!("__code_model_small_", "1"),
        defined!("__gnu_linux__", "1"),
        defined!("__k8", "1"),
        defined!("__k8__", "1"),
        defined!("__linux", "1"),
        defined!("__linux__", "1"),
        defined!("__llvm__", "1"),
        defined!("__seg_fs", "__attribute__((address_space(257)))"),
        defined!("__seg_gs", "__attribute__((address_space(256)))"),
        defined!("__tune_k8__", "1"),
        defined!("__unix", "1"),
        defined!("__unix__", "1"),
        defined!("__x86_64", "1"),
        defined!("__x86_64__", "1"),
        defined!("linux", "1"),
        defined!("unix", "1"),
    ]
}

pub fn get_sys_paths() -> Vec<String> {
    // echo | gcc -Wp,-v -x c++ - -fsyntax-only
    vec![
        "/usr/include/c++/9".to_string(),
        "/usr/include/x86_64-linux-gnu/c++/9".to_string(),
        "/usr/include/c++/9/backward".to_string(),
        "/usr/lib/gcc/x86_64-linux-gnu/9/include".to_string(),
        "/usr/local/include".to_string(),
        "/usr/lib/gcc/x86_64-linux-gnu/9/include-fixed".to_string(),
        "/usr/include/x86_64-linux-gnu".to_string(),
        "/usr/include".to_string(),
    ]
}
