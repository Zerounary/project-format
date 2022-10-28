package com.cp.melon.adapter.api.vo;

import com.cp.melon.adapter.api.convert.{{upperCamel name}}UpdateVO2BOConvert;
import com.cp.melon.entity.{{upperCamel name}}BO;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.mapstruct.factory.Mappers;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class {{upperCamel name}}UpdateVO{
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}

    public {{upperCamel name}}BO toBO() {
        {{upperCamel name}}UpdateVO2BOConvert mapper = Mappers.getMapper({{upperCamel name}}UpdateVO2BOConvert.class);
        return mapper.vo2bo(this);
    }
}
