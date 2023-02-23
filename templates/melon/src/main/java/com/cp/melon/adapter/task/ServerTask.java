package com.cp.melon.adapter.task;

import cn.hutool.core.util.StrUtil;
import com.cp.melon.adapter.db.dao.ScheduleCornDAO;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.EnableScheduling;
import org.springframework.scheduling.annotation.SchedulingConfigurer;
import org.springframework.scheduling.config.ScheduledTaskRegistrar;
import org.springframework.scheduling.support.CronTrigger;
import org.springframework.stereotype.Component;

/**
 * @Author sc
 * @Date 2023/2/21 15:40
 */
@Slf4j
@Component
@EnableScheduling
public class ServerTask implements SchedulingConfigurer {

    @Autowired
    private ScheduleCornDAO scheduleCornDAO;

    @Override
    public void configureTasks(ScheduledTaskRegistrar scheduledTaskRegistrar) {

{{#each tables }}
{{#if cron }}
        scheduledTaskRegistrar.addTriggerTask(() -> {{{name}}}(),
                triggerContext -> {
                    String taskName = "{{{name}}}";
                    String cron = scheduleCornDAO.getCron(taskName);
                    log.info("cron={}", cron);
                    if(StrUtil.isBlank(cron)){
                        log.error("定时任务【{}】的cron不存在", taskName);
                    }
                    return new CronTrigger(cron).nextExecutionTime(triggerContext);
                });
        
{{/if}}
{{/each}}
    }

{{#each tables }}
{{#if cron }}
    // 冻结客户
    private void {{{name}}}() {
        log.info("【{}】定时任务启动", "{{{name}}}");
    }
{{/if}}
{{/each}}
}
