package com.cp.melon.adapter.api.auth;

import com.cp.melon.adapter.auth.Const;
import com.cp.melon.usecase.exception.UnAuthException;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.HttpSession;

/**
 * @Author sc
 * @Date 2023/1/11 10:39
 */
@Component
public class AuthAccessInterceptor implements HandlerInterceptor {

    @Value("${auth}")
    private String isOpenAuth;

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
        if(!(handler instanceof HandlerMethod)){
            return true;
        }
        WebAuth webAuth = ((HandlerMethod) handler).getMethodAnnotation(WebAuth.class);
        if(webAuth != null &&  !"0".equals(isOpenAuth)) {
            // 注解不空，需要登录验证
            HttpSession httpSession = request.getSession();
            AuthUser user = Const.getUser(httpSession);
            if(user == null){
                throw new UnAuthException("未登录");
            }
            return true;
        }
        return true;
    }
}
