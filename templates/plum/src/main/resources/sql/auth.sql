select id, name, password, tenant_id, dept_id, store_id
from sys_user
<where>
and tenant_id = #{tenantId, jdbcType=INTEGER}
<if test="name != null">
and name = #{name, jdbcType=VARCHAR}
</if>
</where>
