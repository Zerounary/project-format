package com.cp.melon.adapter.api.controller;


import com.alibaba.fastjson2.JSONObject;
import com.cp.melon.adapter.api.vo.Resp;
import com.cp.melon.adapter.api.vo.RespPage;
import com.cp.melon.adapter.db.SqlMapperService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

/**
 * @author jicg on 2021/1/6
 */
@RestController
@Slf4j
public class SqlMapperController {
    @Autowired
    private SqlMapperService sqlMapper;

    //列表查询接口
    @PutMapping("/{table}/list")
    public RespPage<JSONObject> query(@RequestBody(required = false) JSONObject request, @PathVariable("table") String table) {
        String fileName = table + "-list";
        RespPage<JSONObject> list = sqlMapper.list(fileName, request);
        return list;
    }

    //详情查询接口
    @PutMapping("/{table}/detail")
    public Resp<JSONObject> detail(@RequestBody(required = false) JSONObject request, @PathVariable("table") String table) {
        String fileName = table + "-detail";
        Resp<JSONObject> detail = sqlMapper.detail(fileName, request);
        return detail;
    }

    @PutMapping("/{table}/noPage")
    public RespPage<JSONObject> noPage(@RequestBody(required = false) JSONObject request, @PathVariable("table") String table) {
        String fileName = table + "-list";
        RespPage<JSONObject> list = sqlMapper.noPage(fileName, request);
        return list;
    }

}

