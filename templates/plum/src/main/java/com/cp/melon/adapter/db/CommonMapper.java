package com.cp.melon.adapter.db;

import com.baomidou.mybatisplus.core.conditions.Wrapper;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.baomidou.mybatisplus.core.toolkit.Constants;
import org.apache.ibatis.annotations.Param;

import java.util.List;

/**
 * @Author sc
 * @Date 2023/5/29 11:34
 */
public interface CommonMapper<T> extends BaseMapper<T> {
    /**
     * 根据 entity 条件，查询一条记录，并锁定
     *
     * @param queryWrapper 实体对象封装操作类（可以为 null）
     */
    T selectOneForUpdate(@Param(Constants.WRAPPER) Wrapper<T> queryWrapper);


    /**
     * 根据 entity 条件，查询全部记录，并锁定
     *
     * @param queryWrapper 实体对象封装操作类（可以为 null）
     */
    List<T> selectListForUpdate(@Param(Constants.WRAPPER) Wrapper<T> queryWrapper);
}
