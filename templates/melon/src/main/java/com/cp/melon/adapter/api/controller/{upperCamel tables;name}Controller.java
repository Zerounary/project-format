package com.cp.melon.adapter.api.controller;

import cn.hutool.core.collection.CollUtil;
import cn.hutool.core.collection.ListUtil;
import cn.hutool.core.util.StrUtil;
import com.baomidou.mybatisplus.core.conditions.AbstractWrapper;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.cp.melon.adapter.api.vo.*;
import com.cp.melon.adapter.service.I{{upperCamel name}}Service;
import com.cp.melon.adapter.db.dao.{{upperCamel name}}DAO;
import com.cp.melon.entity.{{upperCamel name}}BO;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Date;
import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import javax.websocket.server.PathParam;

@RestController
@RequestMapping("/api/{{camel name}}")
public class {{upperCamel name}}Controller{

    @Autowired
    private I{{upperCamel name}}Service {{camel name}}Service;


    @GetMapping("/{id}")
    public Resp<{{upperCamel name}}VO> find{{upperCamel name}}ById( @PathVariable("id") Long id){
        {{upperCamel name}}BO bo = {{camel name}}Service.getById(id);
        {{upperCamel name}}VO vo = {{upperCamel name}}VO.fromBO(bo);
        return Resp.ok(vo);
    }

    @PostMapping("/one")
    public Resp<{{upperCamel name}}VO> find{{upperCamel name}}One(@RequestBody {{upperCamel name}}QueryVO queryVO){
        QueryChainWrapper<{{upperCamel name}}BO> query = {{camel name}}Service.query();
        AbstractWrapper<{{upperCamel name}}BO, String, QueryWrapper<{{upperCamel name}}BO>> wrapper = queryVO.toWrapper(query);
        {{upperCamel name}}BO bo = {{camel name}}Service.getOne(wrapper);
        {{upperCamel name}}VO vo = {{upperCamel name}}VO.fromBO(bo);
        return Resp.ok(vo);
    }

    @PostMapping("/list")
    public Resp<List<{{upperCamel name}}VO>> find{{upperCamel name}}List(@RequestBody {{upperCamel name}}QueryVO queryVO){
        QueryChainWrapper<{{upperCamel name}}BO> query = {{camel name}}Service.query();
        AbstractWrapper<{{upperCamel name}}BO, String, QueryWrapper<{{upperCamel name}}BO>> wrapper = queryVO.toWrapper(query);
        List<{{upperCamel name}}BO> bos = {{camel name}}Service.list(wrapper);
        List<{{upperCamel name}}VO> vos = bos.stream().map(bo -> {{upperCamel name}}VO.fromBO(bo)).collect(Collectors.toList());
        return Resp.ok(vos);
    }

    @PostMapping("/page")
    public RespPage<{{upperCamel name}}VO> find{{upperCamel name}}Page(@RequestBody {{upperCamel name}}QueryVO queryVO, @RequestParam("current") Long currentPage, @RequestParam("size") Long pageSize){
        QueryChainWrapper<{{upperCamel name}}BO> query = {{camel name}}Service.query();
        AbstractWrapper<{{upperCamel name}}BO, String, QueryWrapper<{{upperCamel name}}BO>> wrapper = queryVO.toWrapper(query);
        Page<{{upperCamel name}}BO> page = new Page(currentPage, pageSize);
        Page<{{upperCamel name}}BO> pageResult = {{camel name}}Service.page(page, wrapper);
        List<{{upperCamel name}}VO> vos = pageResult.getRecords().stream().map(bo -> {{upperCamel name}}VO.fromBO(bo)).collect(Collectors.toList());
        return RespPage.ok(currentPage, pageSize, pageResult.getTotal(), vos);
    }

    @PostMapping
    public Resp<{{upperCamel name}}VO> create{{upperCamel name}}(@RequestBody {{upperCamel name}}CreateVO createVO){
        {{upperCamel name}}BO {{upperCamel name}}BO = createVO.toBO();
        boolean result = {{camel name}}Service.save({{upperCamel name}}BO);
        if(result){
            return Resp.ok({{upperCamel name}}VO.fromBO({{upperCamel name}}BO));
        }else {
            return Resp.fail(1, "新增失败");
        }
    }

    @PostMapping("/batch")
    public Resp<List<Long>> create{{upperCamel name}}Batch(@RequestBody List<{{upperCamel name}}CreateVO> createVOList){
        if(CollUtil.isNotEmpty(createVOList)){
            List<{{upperCamel name}}BO> bos = createVOList.stream().map({{upperCamel name}}CreateVO::toBO).collect(Collectors.toList());
            boolean result = {{camel name}}Service.saveBatch(bos, 100);
            if(result){
                List<Long> ids = bos.stream().map(e -> e.getId()).collect(Collectors.toList());
                return Resp.ok(ids);
            }else {
                return Resp.fail(1, "新增失败");
            }
        }
        return Resp.ok(null);
    }

    @PatchMapping
    public Resp<{{upperCamel name}}VO> update{{upperCamel name}}Any(@RequestBody {{upperCamel name}}UpdateVO updateVO){
        boolean result = {{camel name}}Service.updateById(updateVO.toBO());
        if(result) {
            return find{{upperCamel name}}ById(updateVO.getId());
        }else {
            return Resp.fail(1, "更新失败");
        }
    }

    @DeleteMapping("/{ids}")
    public Resp<Boolean> delete{{upperCamel name}}ByIds(@PathVariable("ids") String ids){
        if(ids.matches("([\\d]+,?)+")){
            ids = StrUtil.removeSuffix(ids, ",");
            long[] longs = StrUtil.splitToLong(ids, ",");
            List<Long> idList = CollUtil.newArrayList();
            CollUtil.addAll(idList, longs);
            boolean result = {{camel name}}Service.removeBatchByIds(idList);
            return Resp.ok(result);
        }else {
            return Resp.fail(1, "ids参数异常");
        }
    }
}