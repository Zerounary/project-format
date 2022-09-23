SELECT  a
       ,b
       ,123
       ,myfunc(b)
FROM table_1
WHERE a > b
AND b < 100
ORDER BY a DESC, b