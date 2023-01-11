package com.cp.melon.adapter.auth;

import com.cp.melon.adapter.api.auth.AuthUser;

import javax.servlet.http.HttpSession;

/**
 * @Author sc
 * @Date 2022/10/28 17:35
 */
public class Const {
    public static final String USER = "user";
    public static AuthUser getUser(HttpSession session) {
        AuthUser user = (AuthUser) session.getAttribute(Const.USER);
        return user;
    }
}
