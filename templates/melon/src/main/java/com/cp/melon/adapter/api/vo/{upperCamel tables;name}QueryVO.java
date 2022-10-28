package com.cp.melon.adapter.api.vo;

import com.baomidou.mybatisplus.core.conditions.AbstractWrapper;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.extension.conditions.query.QueryChainWrapper;
import com.cp.melon.entity.{{upperCamel name}}BO;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class {{upperCamel name}}QueryVO{
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}

    public AbstractWrapper<{{upperCamel name}}BO, String, QueryWrapper<{{upperCamel name}}BO>> toWrapper(QueryChainWrapper<{{upperCamel name}}BO> query) {
        query.like("name", name);
        return query.getWrapper();
    }
}
