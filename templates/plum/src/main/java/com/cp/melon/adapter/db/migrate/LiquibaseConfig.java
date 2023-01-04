package com.cp.melon.adapter.db.migrate;

import liquibase.integration.spring.SpringLiquibase;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import javax.sql.DataSource;

/**
 * @Author sc
 * @Date 2022/10/26 22:43
 */
@Configuration
@Slf4j
public class LiquibaseConfig {

    @Bean
    public SpringLiquibase liquibase(DataSource dataSource){
        log.info("Start Default Database Liquibase Update");
        SpringLiquibase liquibase = new SpringLiquibase();
        liquibase.setDataSource(dataSource);
        liquibase.setChangeLog("classpath:master.xml");
        liquibase.setContexts("dev,default,production");
        liquibase.setShouldRun(true);
        return liquibase;
    }
}