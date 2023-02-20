package com.cp.melon.entity;

import com.baomidou.mybatisplus.annotation.FieldFill;
import com.baomidou.mybatisplus.annotation.TableField;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.Date;

/**
 * @Author sc
 * @Date 2023/2/20 11:30
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class BaseBO {
    private static final long serialVersionUID = 1;

    @TableField(fill = FieldFill.INSERT)
    private Date createdTime;


    @TableField(fill = FieldFill.INSERT_UPDATE)
    private Date updatedTime;
}
