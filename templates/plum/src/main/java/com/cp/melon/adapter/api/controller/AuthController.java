package com.cp.melon.adapter.api.controller;

import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.cp.melon.adapter.service.IUserService;
import com.cp.melon.entity.UserBO;
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
    private IUserService userService;

    @GetMapping("/login")
    public String setSession(HttpServletRequest request, String username, String password) {
        HttpSession session = request.getSession();
        QueryChainWrapper<UserBO> query = userService.query();
        query.eq("name", username);
        UserBO userBO = userService.getOne(query.getWrapper());
        if(userBO != null && userBO.getPassword().equals(password)){
            session.setAttribute(username, userBO);
            return "success";
        }else {
            return "fail";
        }
    }

    // @methodDesc: 功能描述:(从Session获取值)

    @RequestMapping("/getSession")
    public String getSession(HttpServletRequest request, String sessionKey) {
        HttpSession session =null;
        try {
            session = request.getSession();
        } catch (Exception e) {
            e.printStackTrace();
        }
        String value=null;
        if(session!=null){
            value = (String) session.getAttribute(sessionKey);
        }
        return "sessionValue:" + value;
    }

}

