package com.cp.melon.adapter.api.config;

import com.cp.melon.adapter.api.auth.AuthAccessInterceptor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

/**
 * @Author sc
 * @Date 2023/1/11 10:43
 */
@Configuration
public class WebConfig implements WebMvcConfigurer {
    @Autowired
    private AuthAccessInterceptor authAccessInterceptor;

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(authAccessInterceptor);
        WebMvcConfigurer.super.addInterceptors(registry);
    }
}
