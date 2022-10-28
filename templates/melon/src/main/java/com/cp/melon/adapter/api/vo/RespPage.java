package com.cp.melon.adapter.api.vo;

import com.alibaba.fastjson2.JSONObject;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import lombok.Data;

import java.util.List;

/**
 * @author jicg on 2021/1/5
 */
@Data
public class RespPage<T> extends Page<T> {

    public RespPage(){};
    public RespPage(long current, long size) {
        super(current, size);
    }

    public static<T> RespPage<T> ok(Long current, Long size, Long total, List<T> records) {
        RespPage<T> resp = new RespPage<T>();
        resp.current = current;
        resp.size = size;
        resp.total = total;
        resp.records = records;
        return resp;
    }
}

