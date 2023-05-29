package com.cp.melon.adapter.db;

import com.baomidou.mybatisplus.core.conditions.Wrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;

import java.util.List;

/**
 * @Author sc
 * @Date 2023/5/29 11:41
 */
public class CommonServiceImpl<M extends CommonMapper<T>, T> extends ServiceImpl<M, T> {


    public T getOneForUpdate(Wrapper<T> queryWrapper) {
        return baseMapper.selectOneForUpdate(queryWrapper);
    }


    public List<T> listForUpdate(Wrapper<T> queryWrapper) {
        return baseMapper.selectListForUpdate(queryWrapper);
    }
}

