package com.cp.melon.adapter.api.vo;

import com.alibaba.fastjson2.JSONObject;
import com.baomidou.mybatisplus.core.metadata.IPage;
import lombok.Data;

import java.util.List;

/**
 * @author jicg on 2021/1/5
 */
@Data
public class RespPage<T> {
    private int code;
    private String msg;
    private long total;
    private JSONObject sumResult;
    private long totalPage;
    private long currentPage;
    private long pageSize;
    private List<T> data;

    public RespPage() {
    }
    public RespPage(long currentPage) {
        this.currentPage = currentPage;
        this.pageSize = 30;
    }

    public RespPage(long currentPage, long pageSize) {
        this.currentPage = currentPage;
        this.pageSize = pageSize;
    }


    //    public static RespPage<String> fail(int code, String msg) {
//        RespPage<String> resp = new RespPage<>();
//        resp.code = code;
//        resp.msg = msg;
//        resp.data = "";
//        return resp;
//    }

    public static <T> RespPage<T> ok(IPage<T> t) {
        RespPage<T> resp = new RespPage<T>();
        resp.code = 0;
        resp.msg = "ok";
        resp.data = t.getRecords();
        resp.pageSize = t.getSize();
        resp.currentPage = t.getCurrent();
        resp.totalPage = t.getPages();
        resp.total = t.getTotal();
        return resp;
    }
}



