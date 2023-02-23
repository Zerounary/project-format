package com.cp.melon.adapter.db.dao;

import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Select;
import org.springframework.stereotype.Repository;

/**
 * @Author sc
 * @Date 2023/2/21 15:32
 */
@Repository
@Mapper
public interface ScheduleCornDAO {

    @Select("select cron from SYS_SCHEDULE_CRON where name = #{name}")
    public String getCron(String name);
}
