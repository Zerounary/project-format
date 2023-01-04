package com.cp.melon.adapter.api.vo;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import com.cp.melon.adapter.api.convert.{{upperCamel name}}VO2BOConvert;
import com.cp.melon.entity.{{upperCamel name}}BO;
import org.mapstruct.factory.Mappers;

import java.math.BigDecimal;
import java.util.Date;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class {{upperCamel name}}VO{
    {{#each columns}}
    private {{type}} {{camel name}};
    {{/each}}

    public static {{upperCamel name}}VO fromBO({{upperCamel name}}BO bo){
        {{upperCamel name}}VO2BOConvert mapper = Mappers.getMapper({{upperCamel name}}VO2BOConvert.class);
        return mapper.bo2vo(bo);
    }

    public {{upperCamel name}}BO toBO() {
        {{upperCamel name}}VO2BOConvert mapper = Mappers.getMapper({{upperCamel name}}VO2BOConvert.class);
        return mapper.vo2bo(this);
    }
}
