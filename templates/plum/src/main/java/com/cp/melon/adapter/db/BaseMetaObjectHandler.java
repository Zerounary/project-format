package com.cp.melon.adapter.db;

import com.baomidou.mybatisplus.core.handlers.MetaObjectHandler;
import com.cp.melon.adapter.api.auth.AuthUser;
import com.cp.melon.adapter.auth.Const;
import org.apache.ibatis.reflection.MetaObject;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import javax.servlet.http.HttpSession;
import java.time.LocalDateTime;
import java.util.Date;

/**
 * @Author sc
 * @Date 2023/1/11 11:26
 */

@Component
public class BaseMetaObjectHandler implements MetaObjectHandler {
    @Autowired
    private HttpSession session;
    @Override
    public void insertFill(MetaObject metaObject) {
        this.strictInsertFill(metaObject, "created_time", Date.class, new Date());
        this.strictInsertFill(metaObject, "updated_time", Date.class, new Date());
        AuthUser user = Const.getUser(session);
        if(user == null) return;
        this.strictInsertFill(metaObject, "created", Long.class, user.getId());
        this.strictInsertFill(metaObject, "updated", Long.class, user.getId());
    }

    @Override
    public void updateFill(MetaObject metaObject) {
        this.strictInsertFill(metaObject, "updated_time", Date.class, new Date());
        AuthUser user = Const.getUser(session);
        if(user == null) return;
        this.strictInsertFill(metaObject, "updated", Long.class, user.getId());
    }
}
