package com.cp.melon.entity.common;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

/**
 * @Author sc
 * @Date 2022/10/28 15:45
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class BatchResult {
    private List<Long> ids;
    private List<ErrorBO> errors;
}
