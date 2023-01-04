package com.cp.melon.adapter.api.config;

import com.github.abel533.sql.SqlMapper;
import org.apache.ibatis.session.SqlSession;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.DefaultResourceLoader;
import org.springframework.core.io.ResourceLoader;

/**
 * @Author sc
 * @Date 2022/10/28 17:42
 */
@Configuration
public class CoreConfig {

    @Autowired
    SqlSession sqlSession;

    @Bean
    public SqlMapper sqlMapper() {
        return new SqlMapper(sqlSession);
    }


    @Bean
    public ResourceLoader resourceLoader() {
        return new DefaultResourceLoader();
    }
}
