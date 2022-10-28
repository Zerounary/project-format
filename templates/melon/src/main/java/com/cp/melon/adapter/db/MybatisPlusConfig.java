package com.cp.melon.adapter.db;

import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.extension.incrementer.OracleKeyGenerator;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.PaginationInnerInterceptor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/**
 * @Author sc
 * @Date 2022/10/28 10:29
 */
@Configuration
@Slf4j
public class MybatisPlusConfig {

    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {
        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();
        interceptor.addInnerInterceptor(new PaginationInnerInterceptor(DbType.ORACLE));
        return interceptor;
    }

    @Bean
    public OracleKeyGenerator oracleKeyGenerator() {
        return new OracleKeyGenerator() {
            @Override
            public String executeSql(String incrementerName) {
                return "SELECT get_sequences('" + incrementerName + "') FROM DUAL";
            }
        };
    }
}

