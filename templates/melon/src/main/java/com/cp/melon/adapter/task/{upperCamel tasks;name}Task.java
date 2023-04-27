package com.cp.melon.adapter.task;

import cn.hutool.core.collection.CollUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.json.JSONArray;
import cn.hutool.json.JSONObject;
import cn.hutool.json.JSONUtil;
import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.cp.melon.adapter.db.dao.ScheduleCornDAO;
import com.cp.melon.adapter.service.I{{upperCamel name}}Service;
import com.cp.melon.adapter.utils.WeimobHttpUtil;
import com.cp.melon.entity.{{upperCamel name}}BO;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.scheduling.Trigger;
import org.springframework.scheduling.support.CronTrigger;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.stream.Collectors;

/**
 * @Author sc
 * @Date 2023/2/23 14:23
 */
@Slf4j
@Component
public class {{upperCamel name}}Task {

    private String taskName = "{{{snake name}}}";

    @Value("${ {{{snake name}}} }")
    private String URL;

    @Autowired
    private WeimobHttpUtil weimobHttpUtil;

    @Autowired
    I{{upperCamel name}}Service {{camel name}}Service;

    @Autowired
    private ScheduleCornDAO scheduleCornDAO;

    public  Runnable get_process() {
        return () -> {
            process();
        };
    }

    public Trigger get_trigger() {
        return triggerContext -> {
            String cron = scheduleCornDAO.getCron(taskName);
            log.info("cron={}", cron);
            if(StrUtil.isBlank(cron)){
                log.error("定时任务【{}】的cron不存在", taskName);
            }
            return new CronTrigger(cron).nextExecutionTime(triggerContext);
        };
    }

    private void process() {
        log.info("【{}】定时任务启动", taskName);
        // 查出全部需要推送的数据
        QueryChainWrapper<{{upperCamel name}}BO> query = {{camel name}}Service.query();
        query.eq("is_propel", "N");
        List<{{upperCamel name}}BO> propelList = query.list();
        // 分批次推送
        for({{upperCamel name}}BO bo : propelList ){
            // 装配参数，推送到指定接口
            String response = weimobHttpUtil.post(URL, JSONUtil.toJsonStr(bo));

            // 默认所有失败
            Boolean isSuccess = false;
            if(JSONUtil.isJson(response)){
                JSONObject json = JSONUtil.parseObj(response);
                isSuccess = json.getJSONObject("data").getBool("status");
            }

            // 回写成功推送状态
            if(isSuccess){
                bo.setIsPropel("Y");
            }else {
                bo.setIsPropel("E");
            }
            {{camel name}}Service.updateById(bo);
        }

        log.info("【{}】定时任务完成", taskName);
    }

}
