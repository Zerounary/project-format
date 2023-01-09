package com.cp.melon.usecase.exception;

/**
 * @Author sc
 * @Date 2022/10/28 14:09
 */
public class AppException extends RuntimeException {
    private static final long serialVersionUID = 1L;

    public AppException(String message) {
        super(message);
    }
}