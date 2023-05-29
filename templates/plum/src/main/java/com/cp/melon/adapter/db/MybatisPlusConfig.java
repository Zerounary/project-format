package com.cp.melon.adapter.db;

import cn.hutool.core.collection.CollUtil;
import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.extension.incrementer.PostgreKeyGenerator;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.plugins.handler.TenantLineHandler;
import com.baomidou.mybatisplus.extension.plugins.inner.PaginationInnerInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.TenantLineInnerInterceptor;
import com.cp.melon.adapter.api.auth.AuthUser;
import com.cp.melon.adapter.auth.Const;
import com.cp.melon.usecase.exception.UnAuthException;
import lombok.extern.slf4j.Slf4j;
import net.sf.jsqlparser.expression.Expression;
import net.sf.jsqlparser.expression.LongValue;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import javax.servlet.http.HttpSession;
import java.util.ArrayList;


/**
 * @Author sc
 * @Date 2022/10/28 10:29
 */
@Configuration
@Slf4j
public class MybatisPlusConfig {

    @Autowired
    HttpSession session;

    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {
        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();
        interceptor.addInnerInterceptor(new PaginationInnerInterceptor(DbType.POSTGRE_SQL));
        interceptor.addInnerInterceptor(new TenantLineInnerInterceptor(new TenantLineHandler() {
            @Override
            public Expression getTenantId() {
                AuthUser user = Const.getUser(session);
                if(user == null) {
                    throw new UnAuthException("用户未登录");
                }
                return new LongValue(user.getTenantId());
            }

            // 这是 default 方法,默认返回 false 表示所有表都需要拼多租户条件
            @Override
            public boolean ignoreTable(String tableName) {
                ArrayList<Object> whitelist = CollUtil.newArrayList();
                AuthUser user = Const.getUser(session);
                return CollUtil.contains(whitelist, tableName) || user == null;
            }
        }));
        return interceptor;
    }

     @Bean
     public PostgreKeyGenerator postgreKeyGenerator() {
         return new PostgreKeyGenerator() {
             @Override
             public String executeSql(String incrementerName) {
                 return "SELECT nextval('seq_" + incrementerName + "')";
             }
         };
     }

    /**
     * 自定义 SqlInjector
     * 里面包含自定义的全局方法
     */
    @Bean
    public CustomerSqlInjector myLogicSqlInjector() {
        return new CustomerSqlInjector();
    }
}

