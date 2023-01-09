package com.cp.melon.adapter.api.controller;

import cn.hutool.core.collection.CollUtil;
import cn.hutool.core.util.StrUtil;
import com.baomidou.mybatisplus.core.conditions.AbstractWrapper;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.cp.melon.adapter.api.vo.*;
import com.cp.melon.adapter.service.I{{upperCamel name}}Service;
import com.cp.melon.entity.{{upperCamel name}}BO;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Date;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import com.cp.melon.adapter.utils.QueryParamUtils;

import com.cp.melon.entity.common.BatchResult;
import com.cp.melon.usecase.{{upperCamel name}}Usecase;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/{{camel name}}")
public class {{upperCamel name}}Controller{

    @Autowired
    private I{{upperCamel name}}Service {{camel name}}Service;

    @Autowired
    private {{upperCamel name}}Usecase {{camel name}}Usecase;


    @GetMapping("/{id}")
    public Resp<{{upperCamel name}}VO> find{{upperCamel name}}ById( @PathVariable("id") Long id){
        {{upperCamel name}}BO bo = {{camel name}}Service.getById(id);
        {{upperCamel name}}VO vo = {{upperCamel name}}VO.fromBO(bo);
        return Resp.ok(vo);
    }

    @GetMapping("/one")
    public Resp<{{upperCamel name}}VO> find{{upperCamel name}}One(@RequestParam Map<String, Object> params){
        QueryWrapper wrapper = QueryParamUtils.getEntityWrapper(params, {{upperCamel name}}BO.class);
        {{upperCamel name}}BO bo = {{camel name}}Service.getOne(wrapper);
        {{upperCamel name}}VO vo = {{upperCamel name}}VO.fromBO(bo);
        return Resp.ok(vo);
    }

    @GetMapping("/noPage")
    public Resp<List<{{upperCamel name}}VO>> find{{upperCamel name}}List(@RequestParam Map<String, Object> params){
        QueryWrapper wrapper = QueryParamUtils.getEntityWrapper(params, {{upperCamel name}}BO.class);
        List<{{upperCamel name}}BO> bos = {{camel name}}Service.list(wrapper);
        List<{{upperCamel name}}VO> vos = bos.stream().map(bo -> {{upperCamel name}}VO.fromBO(bo)).collect(Collectors.toList());
        return Resp.ok(vos);
    }

    @GetMapping("/list")
    public Page<{{upperCamel name}}VO> find{{upperCamel name}}Page(@RequestParam Map<String, Object> params, @RequestParam( "currentPage") Long currentPage, @RequestParam( "pageSize") Long pageSize){
        QueryWrapper wrapper = QueryParamUtils.getEntityWrapper(params, {{upperCamel name}}BO.class);
        Page<{{upperCamel name}}BO> page = new Page(currentPage, pageSize);
        Page<{{upperCamel name}}BO> pageResult = {{camel name}}Service.page(page, wrapper);
        List<{{upperCamel name}}VO> vos = pageResult.getRecords().stream().map(bo -> {{upperCamel name}}VO.fromBO(bo)).collect(Collectors.toList());
        Page<{{upperCamel name}}VO> result = new Page(currentPage, pageSize, pageResult.getTotal());
        result.setRecords(vos);
        return result;
    }
    @PostMapping
    public Resp<Long> create{{upperCamel name}}(@RequestBody {{upperCamel name}}CreateVO createVO){
        {{upperCamel name}}BO bo = createVO.toBO();
        Long id = {{camel name}}Usecase.create(bo);
        return Resp.ok(id);
    }

    @PostMapping("/batch")
    public Resp<BatchResult> create{{upperCamel name}}Batch(@RequestBody List<{{upperCamel name}}CreateVO> createVOList){
        if(CollUtil.isNotEmpty(createVOList)){
            List<{{upperCamel name}}BO> bos = createVOList.stream().map({{upperCamel name}}CreateVO::toBO).collect(Collectors.toList());
            BatchResult result = {{camel name}}Usecase.createBatch(bos);
            return Resp.ok(result);
        }
        return Resp.ok(null);
    }

    @PatchMapping
    public Resp<Boolean> update{{upperCamel name}}Any(@RequestBody {{upperCamel name}}UpdateVO updateVO){
        Boolean result = {{camel name}}Usecase.update(updateVO.toBO());
        return Resp.ok(result);
    }

    @DeleteMapping("/{ids}")
    public Resp<BatchResult> delete{{upperCamel name}}ByIds(@PathVariable("ids") String ids){
        if(ids.matches("([\\d]+,?)+")){
            ids = StrUtil.removeSuffix(ids, ",");
            long[] longs = StrUtil.splitToLong(ids, ",");
            BatchResult batchResult = {{camel name}}Usecase.deleteBatch(longs);
            return Resp.ok(batchResult);
        }else {
            return Resp.fail(1, "ids参数异常");
        }
    }
}