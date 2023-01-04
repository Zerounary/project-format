package com.cp.melon.adapter.db;

import cn.hutool.core.collection.CollUtil;
import cn.hutool.core.collection.ListUtil;
import cn.hutool.core.util.CharsetUtil;
import cn.hutool.core.util.NumberUtil;
import cn.hutool.core.util.StrUtil;
import com.alibaba.fastjson2.JSONObject;
import com.baomidou.mybatisplus.core.toolkit.StringUtils;
import com.cp.melon.adapter.api.vo.Resp;
import com.cp.melon.adapter.api.vo.RespPage;
import com.cp.melon.adapter.auth.Const;
import com.cp.melon.entity.UserBO;
import com.github.abel533.sql.SqlMapper;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.io.IOUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.ResourceLoader;
import org.springframework.stereotype.Service;

import javax.servlet.http.HttpSession;
import java.io.IOException;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Objects;

/**
 * @Author sc
 * @Date 2022/10/28 17:18
 */
@Service
@Slf4j
public class SqlMapperService {
    @Autowired
    SqlMapper sqlMapper;
    @Autowired
    ResourceLoader resourceLoader;
    @Autowired
    HttpSession session;

    public RespPage<JSONObject> noPage(String fileName, JSONObject request) {
        return this.list(fileName, true, request);
    }

    public RespPage<JSONObject> list(String fileName, JSONObject request) {
        return this.list(fileName, false, request);
    }

    public RespPage<JSONObject> list(String fileName, boolean isNoPage, JSONObject request) {
        UserBO user = Const.getUser(session);
        request.put("user", user);
        RespPage<JSONObject> resultList = new RespPage<JSONObject>();
        String sqlContent = getFileContent(fileName);
        Long total = 0L;
        if (StringUtils.isNotBlank(sqlContent)) {
            String totalSql = getListTotalSql(sqlContent);
            // 判断是否有需要预加载到请求参数中的sql语句
            String refReportName = StrUtil.subAfter(fileName, ".", true);
            if (StrUtil.isNotBlank(refReportName)) {
                String recordsSql = getCleanSqlContent(getFileContent(refReportName));
                List<JSONObject> maps = sqlMapper.selectList(recordsSql, request, JSONObject.class);
                request.put("_" + StrUtil.removeSuffix(refReportName, "-list"), maps);
            }
            JSONObject sumResult = sqlMapper.selectOne(totalSql, request, JSONObject.class);
            total = sumResult.getLongValue("total");
            resultList.setSumResult(sumResult);
            resultList.setTotal(total);
            if (isNoPage) {
                request.put("currentPage", 1);
                request.put("pageSize", total);
            }
            request = pageHandle(request);
            resultList.setCurrentPage(request.getLongValue("currentPage"));
            resultList.setPageSize(request.getLongValue("pageSize"));
        }
        if (!total.equals(0L)) {
            String recordsSql = getListRecordsSql(sqlContent);
            List<LinkedHashMap> records = sqlMapper.selectList(recordsSql, request, LinkedHashMap.class); // 为了前端key和sql顺序一致
            List<JSONObject> collect = CollUtil.newArrayList();
            for (LinkedHashMap record : records) {
                JSONObject js = new JSONObject();
                js.putAll(record);
                collect.add(js);
            }
            resultList.setData(collect);
            resultList.setTotalPage(NumberUtil.count(total.intValue(), request.getLong("pageSize").intValue()));
        } else {
            resultList.setTotal(0L);
            resultList.setTotalPage(0L);
            resultList.setData(ListUtil.empty());
        }

        return resultList;
    }



    public Resp<JSONObject> detail(String fileName, JSONObject request) {
        RespPage<JSONObject> listRst = list(fileName, request);
        if (CollUtil.isNotEmpty(listRst.getData())) {
            JSONObject firstRst = listRst.getData().get(0);
            firstRst.remove("_no");
            return Resp.ok(firstRst);
        }
        return Resp.ok(null);
    }

    /**
     * 对文件内的内容进行简单转换
     * <p>
     * 尖括号符号前后需要有空格
     * 大于等于使用 >=
     * 小于等于使用 <=
     * 小于使用 <
     * 大于使用 >
     * <p>
     * 注释
     * 支持文件内顶行--写文件注释
     *
     * @param fileName
     * @return
     * @throws IOException
     */
    private String getSql(String fileName) throws IOException {
        String pathName = "classpath:sql/".concat(fileName).concat(".sql");
        String sql = IOUtils.toString(resourceLoader.getResource(pathName).getInputStream(), CharsetUtil.UTF_8)
                .replaceAll("--.*\n", "")  // 文件内支持 -- 注释
                .replaceAll(" >= ", "<![CDATA[>=]]>")
                .replaceAll(" <= ", "<![CDATA[<=]]>")
                .replaceAll(" > ", "<![CDATA[>]]>")
                .replaceAll(" < ", "<![CDATA[<]]>");
        return sql;
    }

    /**
     * 执行Sql前先设置请求的分页信息
     *
     * @param request
     * @return
     */
    private JSONObject pageHandle(JSONObject request) {
        if (Objects.isNull(request)) {
            request = new JSONObject();
            request.put("currentPage", 1L);
            request.put("pageSize", 30L);
            JSONObject page = new JSONObject();
            page.put("start", 1);
            page.put("end", 30);
            request.put("page", page);
        } else {
            Long currentPage = request.getLong("currentPage");
            currentPage = Objects.nonNull(currentPage) ? currentPage : 1L;
            Long pageSize = request.getLong("pageSize");
            pageSize = Objects.nonNull(pageSize) ? pageSize : 30L; // 默认30一页
            request.put("currentPage", currentPage);
            request.put("pageSize", pageSize);
            JSONObject page = new JSONObject();
            page.put("start", (currentPage - 1L) * pageSize + 1L);
            page.put("end", currentPage * pageSize);
            request.put("page", page);
        }
        return request;
    }

    /**
     * 根据文件名名读取文件内容
     *
     * @param fileName
     * @return
     */
    private String getFileContent(String fileName) {
        String sql = "";
        try {
            sql = getSql(fileName);
        } catch (IOException e) {
            log.error(fileName + "未找到");
        }
        return sql;
    }

    /**
     * 获取不分页的数据
     * @param sql
     * @return
     */
    private String getCleanSqlContent(String sql) {
        String cleanSql = cleanSqlContent(sql);

        StringBuffer stringBuffer = new StringBuffer();
        stringBuffer.append("<script> ");
        stringBuffer.append(cleanSql);
        stringBuffer.append(" </script>");

        return stringBuffer.toString();
    }

    /**
     * 获取查询内容数据的sql
     *
     * @param sql
     * @return
     */
    private String getListRecordsSql(String sql) {

        String cleanSql = cleanSqlContent(sql);

        StringBuffer stringBuffer = new StringBuffer();
        stringBuffer.append("<script> ");
        stringBuffer.append(" SELECT * FROM ( SELECT rownum as \"_no\", \"_no\".* FROM ( ");
        stringBuffer.append(cleanSql);
        stringBuffer.append("<if test=\"sorter != null and sorter.size() != 0\"> ");
        stringBuffer.append("order by ");
        stringBuffer.append("<foreach item=\"order\" index=\"column\" collection=\"sorter\" separator=\",\" > ");
        stringBuffer.append(" \"${column}\" ${order}");
        stringBuffer.append("</foreach> ");
        stringBuffer.append("</if> ");
        stringBuffer.append(" ) \"_no\" where rownum <![CDATA[<=]]> #{page.end}) ");
        stringBuffer.append(" WHERE \"_no\" <![CDATA[>=]]> #{page.start} ");
        stringBuffer.append("</script>");

        return stringBuffer.toString();
    }

    /**
     * 获取查询内容数据的sql的统计数据
     * 每个查选前第一行第一个字符使用#开头，后面都使用 sql的统计语句写法，
     * 可以将统计信息查询到列表响应中
     *
     * @param sql
     * @return
     */
    private String getListTotalSql(String sql) {

        String sumColumns = getSumColumns(sql);
        String cleanSql = cleanSqlContent(sql);

        StringBuffer stringBuffer = new StringBuffer();
        stringBuffer.append("<script> ");
        stringBuffer.append(" SELECT count(1) as \"total\"");

        if (StringUtils.isNotBlank(sumColumns)) {
            stringBuffer.append(" , ");
            stringBuffer.append(sumColumns);
        }
        stringBuffer.append(" FROM ( ");
        stringBuffer.append(cleanSql);
        stringBuffer.append(" ) ");
        stringBuffer.append("</script>");

        return stringBuffer.toString();
    }

    private String getSumColumns(String sql) {
        if (sql.trim().startsWith("#")) {
            String[] lines = sql.split("\r\n");
            String firstLine = lines[0];
            return firstLine.substring(1);
        } else {
            return "";
        }
    }

    /**
     * 删除所有的sql描述行
     *
     * @param sql
     * @return
     */
    private String cleanSqlContent(String sql) {
        if (sql.trim().startsWith("#")) {
            String[] lines = sql.split("\r\n");
            List<String> otherLines = ListUtil.toList(lines).subList(1, lines.length);
            return CollUtil.join(otherLines, "\r\n");
        } else {
            return sql;
        }
    }
}
