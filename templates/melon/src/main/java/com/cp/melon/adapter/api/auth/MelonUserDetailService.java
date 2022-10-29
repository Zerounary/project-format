package com.cp.melon.adapter.api.auth;

import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.cp.melon.adapter.service.IUserService;
import com.cp.melon.entity.UserBO;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.authority.AuthorityUtils;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

/**
 * @Author sc
 * @Date 2022/10/29 21:22
 */
@Service
public class MelonUserDetailService implements UserDetailsService {

    @Autowired
    private PasswordEncoder passwordEncoder;
    @Autowired
    private IUserService userService;

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        QueryChainWrapper<UserBO> query = userService.query();
        query.eq("name", username);
        UserBO userBO = userService.getOne(query.getWrapper());
//        String permissions = "";
//        boolean notLocked = false;
//        AuthUser user = new AuthUser(userBO.getName(), userBO.getPassword(), true, true, true, notLocked, AuthorityUtils.commaSeparatedStringToAuthorityList("user:add"));
//        return user;

//        return new User(username, passwordEncoder.encode("123456"), true, true, true, true, AuthorityUtils.commaSeparatedStringToAuthorityList("user:add"));
//        return new User(userBO.getName(), userBO.getPassword(), true, true, true, true, AuthorityUtils.commaSeparatedStringToAuthorityList("user:add"));
        return new AuthUser(userBO.getName(), userBO.getPassword(), true, true, true, true, AuthorityUtils.commaSeparatedStringToAuthorityList("user:add"));
    }

}
