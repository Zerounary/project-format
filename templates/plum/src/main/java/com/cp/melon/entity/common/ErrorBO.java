package com.cp.melon.entity.common;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @Author sc
 * @Date 2022/10/28 15:31
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class ErrorBO {
    private int no;
    private String msg;
}
