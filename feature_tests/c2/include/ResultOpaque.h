#ifndef ResultOpaque_H
#define ResultOpaque_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_ErrorEnum_box_ResultOpaque.h"
#include "diplomat_result_box_ResultOpaque_ErrorEnum.h"
#include "diplomat_result_box_ResultOpaque_ErrorStruct.h"
#include "diplomat_result_box_ResultOpaque_void.h"
#include "diplomat_result_void_box_ResultOpaque.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct diplomat_result_ErrorEnum_box_ResultOpaque diplomat_result_ErrorEnum_box_ResultOpaque;
typedef struct diplomat_result_box_ResultOpaque_ErrorEnum diplomat_result_box_ResultOpaque_ErrorEnum;
typedef struct diplomat_result_box_ResultOpaque_ErrorStruct diplomat_result_box_ResultOpaque_ErrorStruct;
typedef struct diplomat_result_box_ResultOpaque_void diplomat_result_box_ResultOpaque_void;
typedef struct diplomat_result_void_box_ResultOpaque diplomat_result_void_box_ResultOpaque;


typedef struct ResultOpaque ResultOpaque;



diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new(int32_t i);
diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_foo();
diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_bar();
diplomat_result_box_ResultOpaque_void ResultOpaque_new_failing_unit();
diplomat_result_box_ResultOpaque_ErrorStruct ResultOpaque_new_failing_struct(int32_t i);
diplomat_result_void_box_ResultOpaque ResultOpaque_new_in_err(int32_t i);
diplomat_result_ErrorEnum_box_ResultOpaque ResultOpaque_new_in_enum_err(int32_t i);
void ResultOpaque_assert_integer(const ResultOpaque* self, int32_t i);
void ResultOpaque_destroy(ResultOpaque* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ResultOpaque_H
