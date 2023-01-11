package com.cp.melon.adapter.api.auth;

import lombok.Data;

import java.io.Serializable;

/**
 * @Author sc
 * @Date 2023/1/11 9:26
 */
@Data
public class AuthUser implements Serializable {
    private static final long serialVersionUID = 1;
    private Long id;
    private String name;
    private String password;
    private Long tenantId;
    private Long storeId;
    private Long deptId;
}
