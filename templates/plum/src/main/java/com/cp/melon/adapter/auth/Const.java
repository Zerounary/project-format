package com.cp.melon.adapter.auth;

import com.cp.melon.entity.UserBO;

import javax.servlet.http.HttpSession;

/**
 * @Author sc
 * @Date 2022/10/28 17:35
 */
public class Const {
    public static final String USER = "user";
    public static UserBO getUser(HttpSession session) {
        UserBO user = (UserBO) session.getAttribute(Const.USER);
        return user;
    }
}
