package com.cp.melon.adapter.task;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.EnableScheduling;
import org.springframework.scheduling.annotation.SchedulingConfigurer;
import org.springframework.scheduling.config.ScheduledTaskRegistrar;
import org.springframework.stereotype.Component;

/**
 * @Author sc
 * @Date 2023/2/21 15:40
 */
@Slf4j
@Component
@EnableScheduling
public class ServerTask implements SchedulingConfigurer {

{{#each tables}}
{{#if cron}}
    @Autowired
    {{upperCamel name}}Task {{camel name}}Task;
{{/if}}
{{/each}}

    @Override
    public void configureTasks(ScheduledTaskRegistrar scheduledTaskRegistrar) {

{{#each tables}}
{{#if cron}}
        scheduledTaskRegistrar.addTriggerTask({{camel name}}Task.get_process(), {{camel name}}Task.get_trigger());
{{/if}}
{{/each}}

    }

}