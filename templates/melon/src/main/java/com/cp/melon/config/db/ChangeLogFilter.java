package com.cp.melon.config.db;

import liquibase.changelog.IncludeAllFilter;

/**
 * @Author sc
 * @Date 2022/4/4 10:45
 */
public class ChangeLogFilter implements IncludeAllFilter {
    @Override
    public boolean include(String changeLogPath) {
        return changeLogPath.endsWith(".sql");
    }
}
