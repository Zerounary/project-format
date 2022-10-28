package com.cp.melon.adapter.api.vo;

import lombok.Data;

/**
 * @author jicg on 2021/1/5
 */
@Data
public class Resp<T> {
    private int code;
    private String msg;
    private T data;

    public static<T> Resp<T> fail(int code, String msg) {
        Resp<T> resp = new Resp<>();
        resp.code = code;
        resp.msg = msg;
        resp.data = null;
        return resp;
    }

    public static <T> Resp<T> ok(T t) {
        Resp<T> resp = new Resp<T>();
        resp.code = 0;
        resp.msg = "ok";
        resp.data = t;
        return resp;
    }
}

