package com.cp.melon.adapter.db;

import com.baomidou.mybatisplus.core.injector.AbstractMethod;
import com.baomidou.mybatisplus.core.injector.DefaultSqlInjector;
import com.baomidou.mybatisplus.core.injector.methods.SelectById;
import com.baomidou.mybatisplus.core.metadata.TableInfo;
import com.cp.melon.adapter.db.methods.SelectListForUpdate;
import com.cp.melon.adapter.db.methods.SelectOneForUpdate;

import java.util.List;

/**
 * @Author sc
 * @Date 2023/5/29 11:30
 */
public class CustomerSqlInjector extends DefaultSqlInjector {

    @Override
    public List<AbstractMethod> getMethodList(Class<?> mapperClass, TableInfo tableInfo) {
        List<AbstractMethod> methodList = super.getMethodList(mapperClass, tableInfo);
        methodList.add(new SelectOneForUpdate());
        methodList.add(new SelectListForUpdate());
        return methodList;
    }
}

