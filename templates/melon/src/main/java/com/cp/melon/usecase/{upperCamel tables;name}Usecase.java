package com.cp.melon.usecase;

import lombok.extern.slf4j.Slf4j;
import cn.hutool.core.collection.CollUtil;
import com.cp.melon.adapter.service.I{{upperCamel name}}Service;
import com.cp.melon.entity.{{upperCamel name}}BO;
import com.cp.melon.entity.common.BatchResult;
import com.cp.melon.entity.common.BusResultBO;
import com.cp.melon.entity.common.ErrorBO;
import com.cp.melon.usecase.exception.NotFoundException;
import com.cp.melon.usecase.exception.UsecaseException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;

/**
 * @Author sc
 * @Date 2022/10/28 14:05
 */
@Slf4j
@Service
public class {{upperCamel name}}Usecase {

    @Autowired
    private I{{upperCamel name}}Service {{camel name}}Service;

    @Transactional
    public Long create({{upperCamel name}}BO bo) {
        if ({{camel name}}Service.save(bo)) {
            return bo.getId();
        }else {
            throw new UsecaseException("新增失败");
        }
    }

    @Transactional
    public BatchResult createBatch(List<{{upperCamel name}}BO> bos) {
        BatchResult result = new BatchResult();
        List<Long> ids = CollUtil.newArrayList();
        List<ErrorBO> errors = CollUtil.newArrayList();
        for (int i = 0; i < bos.size(); i++) {
            {{upperCamel name}}BO bo = bos.get(i);
            try{
                ids.add(create(bo));
            }catch (Exception e){
                errors.add(new ErrorBO(i, e.getMessage()));
            }
        }
        result.setIds(ids);
        result.setErrors(errors);
        return result;
    }

    @Transactional
    public Boolean update({{upperCamel name}}BO bo) {
        if({{camel name}}Service.updateById(bo)) {
            return true;
        }else {
            throw new UsecaseException("更新失败");
        }
    }

    @Transactional
    public BatchResult updateBatch(List<{{upperCamel name}}BO> bos) {
        BatchResult result = new BatchResult();
        List<Long> ids = CollUtil.newArrayList();
        List<ErrorBO> errors = CollUtil.newArrayList();
        for (int i = 0; i < bos.size(); i++) {
            {{upperCamel name}}BO bo = bos.get(i);
            try{
                update(bo);
                ids.add(bo.getId());
            }catch (Exception e){
                errors.add(new ErrorBO(i, e.getMessage()));
            }
        }
        result.setIds(ids);
        result.setErrors(errors);
        return result;
    }

    @Transactional
    public Boolean delete(Long id) {
        if({{camel name}}Service.removeById(id)){
            return true;
        }else {
            throw new UsecaseException("删除失败");
        }
    }

    @Transactional
    public BatchResult deleteBatch(long[] ids){
        BatchResult result = new BatchResult();
        List<Long> idList = CollUtil.newArrayList();
        List<ErrorBO> errors = CollUtil.newArrayList();
        for (int i = 0; i < ids.length; i++) {
            Long id = ids[i];
            try {
                delete(id);
                idList.add(id);
            }catch (Exception e){
                errors.add(new ErrorBO(i, e.getMessage()));
            }
        }
        result.setIds(idList);
        result.setErrors(errors);
        return result;
    }
}
