package com.cp.melon.adapter.api.convert;

{{#each tables}}
import com.cp.melon.adapter.api.vo.{{upperCamel name}}VO;
import com.cp.melon.adapter.api.vo.{{upperCamel name}}CreateVO;
import com.cp.melon.adapter.api.vo.{{upperCamel name}}UpdateVO;
import com.cp.melon.entity.{{upperCamel name}}BO;
{{/each}}
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.factory.Mappers;

/**
 * @Author sc
 * @Date 2023/2/20 13:49
 */
@Mapper
public interface VOConverter {

    VOConverter INSTANCE = Mappers.getMapper(VOConverter.class);

    /*
    @Mapping(source = "id", target = "id", ignore = true)
    @Mapping(source = "id", target = "eid")
    @Mapping(source = "msgBody.membershipCreateSceneType", target = "sceneType")
    @Mapping(source = "msgBody.membershipPlanId", target = "planId")
    @Mapping(source = "msgBody.membershipType", target = "type")
    WeimobCustomerCreateBO customerCreateVO2BO(WeimobCustomerCreateCreateVO vo);
    */

{{#each tables}}
    {{upperCamel name}}BO {{upperCamel name}}VO2BO({{upperCamel name}}VO vo);
    {{upperCamel name}}BO {{upperCamel name}}CreateVO2BO({{upperCamel name}}CreateVO vo);
    {{upperCamel name}}BO {{upperCamel name}}UpdateVO2BO({{upperCamel name}}UpdateVO vo);
{{/each}}
}
