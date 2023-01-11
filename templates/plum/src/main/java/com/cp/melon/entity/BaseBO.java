package com.cp.melon.entity;

import com.baomidou.mybatisplus.annotation.FieldFill;
import com.baomidou.mybatisplus.annotation.TableField;
import lombok.Data;

import java.io.Serializable;
import java.time.LocalDateTime;
import java.util.Date;

@Data
public class BaseBO implements Serializable {
    private static final long serialVersionUID = 1;

    @TableField(fill = FieldFill.INSERT)
    private Long created;

    @TableField(fill = FieldFill.INSERT)
    private Date created_time;

    @TableField(fill = FieldFill.INSERT_UPDATE)
    private Long updated;

    @TableField(fill = FieldFill.INSERT_UPDATE)
    private Date updated_time;
}