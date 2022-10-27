package com.cp.melon.adapter.db.po;

import lombok.Data;
import lombok.Builder;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;
import com.baomidou.mybatisplus.annotation.TableName;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
@TableName("{{prefix}}{{snake name}}")
public class {{upperCamel name}}PO{
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}
}
