 sqlparser
 仅支持标准SQL
 https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_30_numeric_value_function

 # 简介

 `project-format` 项目，主要解决的是利用历史沉淀的项目重新开发新项目时，初始化项目慢，调整内容多等问题。利用模板引擎，将指定目录中的模板文件，通过配置文件转化成所需的，可直接运行的软件项目。

 # Guava

 ## 数据库

 当前版本的数据库状态，记录在db目录，该目录主要用于对比各个表和字段，方便手动编写 `migrations` 脚本来自动自动导库。

 migrations 中的SQL 文件的格式要求 `V{1}_{2}.sql`
 
 `{1}`是版本号，所有文件间版本号不可重复，必须唯一。
 `{2}`是当前版本，所做的升级的名称。

 ## 配置

主要配置如下：

 ```yaml
project:
  template:
    dir: "templates/guava"
    output: "../cardroom"
  data:
    project_name: "cardroom"
    auth: 1
    redis_session: 0
    overview: 1
    whitelist: []
    pages: []
    tables: []
    ddls: []
 ```

 `project.template.dir`：所使用的项目模板路径
 `project.template.output`：项目渲染后，输出的目录
 `project.data`：项目渲染的数据


 ## 项目模板

 在程序运行的时候，会首先获取生成项目的`yaml`文件配置。再根据配置中的模板路径，选择项目模板，遍历项目模板中的所有文件。如果是动态文件名，则会根据命名规则来创建多个文件。

 比如，有一个像这样的文件名`db/{snake tables;name}.sql`的文件
 他的渲染数据`project.data`是这样的
 ```yaml
tables:
  - {
    name: "bigDog"
  }
  - {
    name: "smallDog"
  }
 ```

那么会在`db`目录下，创建两个这样的文件
```text
big_dog.sql
small_dog.sql
```

动态文件名`{snake tables;name}`，`snake` 是文本类型转换的标记,  `tables;name` 指向的是一个列表中的属性。 `tables` 是需要遍历的列表, `name`是需要获取的每个元素的某个属性值。



