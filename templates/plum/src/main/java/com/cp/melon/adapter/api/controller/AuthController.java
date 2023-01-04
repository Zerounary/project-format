package com.cp.melon.adapter.api.controller;

import cn.hutool.core.util.StrUtil;
import com.cp.melon.adapter.api.vo.Resp;
import com.cp.melon.usecase.exception.UnAuthException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.oauth2.provider.token.ConsumerTokenServices;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpServletRequest;

/**
 * @Author sc
 * @Date 2022/10/29 21:57
 */
@RestController
public class AuthController {

    @Autowired
    private ConsumerTokenServices consumerTokenServices;

    @DeleteMapping("signout")
    public Resp<String> signout(HttpServletRequest request) throws UnAuthException {
        String authorization = request.getHeader("Authorization");
        String token = StrUtil.replace(authorization, "bearer ", "");
        if (!consumerTokenServices.revokeToken(token)) {
            throw new UnAuthException("退出登录失败");
        }
        return Resp.ok("退出登录成功");
    }

}
