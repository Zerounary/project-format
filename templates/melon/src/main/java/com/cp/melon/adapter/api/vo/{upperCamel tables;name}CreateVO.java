package com.cp.melon.adapter.api.vo;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.mapstruct.factory.Mappers;
import com.cp.melon.entity.{{upperCamel name}}BO;
import com.cp.melon.adapter.api.convert.{{upperCamel name}}CreateVO2BOConvert;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class {{upperCamel name}}CreateVO{
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}

    public {{upperCamel name}}BO toBO() {
        {{upperCamel name}}CreateVO2BOConvert mapper = Mappers.getMapper({{upperCamel name}}CreateVO2BOConvert.class);
        return mapper.vo2bo(this);
    }
}
