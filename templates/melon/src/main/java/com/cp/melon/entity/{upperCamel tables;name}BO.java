package com.cp.melon.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import com.baomidou.mybatisplus.annotation.*;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
@TableName("{{prefix}}{{snake name}}")
@KeySequence(value ="{{prefix}}{{snake name}}")
public class {{upperCamel name}}BO{
    @TableId(type = IdType.INPUT)
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}
}
