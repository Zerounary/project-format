package com.cp.melon.adapter.api.controller;

import cn.hutool.core.bean.BeanUtil;
import cn.hutool.core.map.MapUtil;
import com.cp.melon.adapter.api.auth.AuthUser;
import com.cp.melon.adapter.api.auth.AuthUserVO;
import com.cp.melon.adapter.api.vo.Resp;
import com.cp.melon.adapter.db.SqlMapperService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpSession;

/**
 * @Author sc
 * @Date 2022/10/29 21:57
 */
@RestController
public class AuthController {

    @Autowired
    private SqlMapperService sqlMapper;

    @GetMapping("/login")
    public Resp login(HttpServletRequest request, String username, String password, String tenantId) {
        HttpSession session = request.getSession();
        AuthUser authUser = sqlMapper
                .selectOne(
                        "auth",
                        MapUtil.builder()
                                .put("name", username)
                                .put("tenantId", Long.parseLong(tenantId))
                                .build(),
                        AuthUser.class);
        if (authUser != null && authUser.getPassword().equals(password)) {
            session.setAttribute("user", authUser);
            return Resp.ok(BeanUtil.toBean(authUser, AuthUserVO.class));
        } else {
            return Resp.fail(401, "请检查用户名密码是否正确");
        }
    }

    // @methodDesc: 功能描述:(从Session获取值)

    @RequestMapping("/logout")
    public Resp logout(HttpServletRequest request) {
        HttpSession session = request.getSession();
        session.setAttribute("user", null);
        return Resp.ok(null);
    }

}

