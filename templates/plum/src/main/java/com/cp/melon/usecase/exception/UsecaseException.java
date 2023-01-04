package com.cp.melon.usecase.exception;

/**
 * @Author sc
 * @Date 2022/10/28 14:09
 */
public class UsecaseException extends RuntimeException {
    private static final long serialVersionUID = 1L;

    public UsecaseException(String message) {
        super(message);
    }
}