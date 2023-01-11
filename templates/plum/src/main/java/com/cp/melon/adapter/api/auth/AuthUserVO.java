package com.cp.melon.adapter.api.auth;

import lombok.Data;

import java.io.Serializable;

/**
 * @Author sc
 * @Date 2023/1/11 9:26
 */
@Data
public class AuthUserVO {
    private Long id;
    private String name;
    private Long storeId;
    private Long deptId;
}
