package com.cp.melon.adapter.api.config;

import cn.hutool.core.util.StrUtil;
import com.cp.melon.adapter.api.vo.Resp;
import com.cp.melon.usecase.exception.UnAuthException;
import com.cp.melon.usecase.exception.UsecaseException;
import lombok.extern.slf4j.Slf4j;
import org.apache.ibatis.exceptions.PersistenceException;
import org.mybatis.spring.MyBatisSystemException;
import org.springframework.dao.DuplicateKeyException;
import org.springframework.jdbc.BadSqlGrammarException;
import org.springframework.jdbc.UncategorizedSQLException;
import org.springframework.validation.BindException;
import org.springframework.validation.ObjectError;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseBody;

import java.util.Optional;
import java.util.concurrent.atomic.AtomicReference;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * @Author sc
 * @Date 2022/10/28 14:20
 */
@Slf4j
@ControllerAdvice
public class GlobalErrorHandler {
    @ExceptionHandler(value = UnAuthException.class)
    @ResponseBody
    public Resp handleException(UnAuthException e) throws Exception {
        log.error("认证超时，请重新登录");
        return Resp.fail(401, "认证超时，请重新登录");
    }

    @ExceptionHandler(value = Throwable.class)
    @ResponseBody
    public Resp handleException(Exception e) throws Exception {
        if(e.getLocalizedMessage().contains("com.cp.melon.usecase.exception.UnAuthException")) {
            log.error("认证超时，请重新登录");
            return Resp.fail(401, "认证超时，请重新登录");
        }
        if(e.getLocalizedMessage().contains("ORA-20201")){
            log.error(e.getLocalizedMessage());
            String oraErr = StrUtil.subBetween(e.getLocalizedMessage(), "ORA-20201:", "ORA-06512:");
            return Resp.fail(1, oraErr);
        }
        if (e instanceof UncategorizedSQLException || e instanceof PersistenceException || e instanceof BadSqlGrammarException
                || e instanceof DuplicateKeyException || e instanceof MyBatisSystemException) {
            String msg = findSqlMsg(e.getLocalizedMessage());
            log.error(msg, e.getCause());
            return Resp.fail(1, msg);
        }
        log.error(e.getLocalizedMessage(), e);
        return Resp.fail(2, "错误：" + e.getLocalizedMessage());
    }

    @ExceptionHandler(value = BindException.class)
    @ResponseBody
    public Resp handleException(BindException e) throws Exception {
        log.error(e.getLocalizedMessage(), e);
        AtomicReference<String> errMsg = new AtomicReference<>(e.getLocalizedMessage());
        Optional<ObjectError> first = e.getAllErrors().stream().findFirst();
        first.ifPresent(it -> errMsg.set(it.getDefaultMessage()));
        return Resp.fail(1, "错误：" + errMsg.get());
    }

    @ExceptionHandler(value = UsecaseException.class)
    @ResponseBody
    public Resp handleException(UsecaseException e) throws Exception {
        log.error(e.getLocalizedMessage(), e);
        return Resp.fail(1, "错误：" + e.getLocalizedMessage());
    }

    public static String findSqlMsg(String message) {
        if (message != null && message.matches("[^*]+Exception[^*]+ORA-[0-9]*:[^*]*")) {
            String pattern = "ORA-[0-9]+:(.*)\n";
            Pattern r = Pattern.compile(pattern);
            Matcher m = r.matcher(message);
            if (m.find()) {
                return m.group().replaceAll("ORA-[0-9]+:", "").trim();
            }
        }
        if (message != null && message.matches("[^*]+java.sql.SQLException:[^*]*")) {
            String pattern = "java.sql.SQLException:(.*)\n";
            Pattern r = Pattern.compile(pattern);
            Matcher m = r.matcher(message);
            if (m.find()) {
                return m.group().replaceAll("java.sql.SQLException:", "").trim();
            }
        }
        return message;
    }
}