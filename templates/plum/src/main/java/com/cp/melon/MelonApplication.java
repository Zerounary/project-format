package com.cp.melon;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan("com.cp.melon.adapter.db.dao")
public class MelonApplication {

    public static void main(String[] args) {
        SpringApplication.run(MelonApplication.class, args);
    }

}
