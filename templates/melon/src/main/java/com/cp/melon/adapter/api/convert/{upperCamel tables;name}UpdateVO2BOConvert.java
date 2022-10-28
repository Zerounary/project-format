package com.cp.melon.adapter.api.convert;

import com.cp.melon.adapter.api.vo.{{upperCamel name}}UpdateVO;
import com.cp.melon.adapter.api.vo.{{upperCamel name}}VO;
import com.cp.melon.entity.{{upperCamel name}}BO;
import org.mapstruct.Mapper;

/**
 * @Author sc
 * @Date 2022/10/28 10:17
 */
@Mapper(componentModel = "spring")
public abstract class {{upperCamel name}}UpdateVO2BOConvert implements IConverter<{{upperCamel name}}UpdateVO, {{upperCamel name}}BO> {
}
