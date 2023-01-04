package com.cp.melon.usecase.exception;

/**
 * @Author sc
 * @Date 2022/10/28 14:10
 */
public class NotFoundException extends UsecaseException {
    private static final long serialVersionUID = 1L;

    public NotFoundException(String message) {
        super(message);
    }
}