package com.cp.melon.adapter.db.migrate;

/**
 * @Author sc
 * @Date 2022/10/26 22:45
 */
import java.util.Comparator;

/**
 * @Author sc
 * @Date 2022/4/4 10:44
 */
public class ChangeLogComparator implements Comparator<String> {
    public static final String INIT = "init";

    public ChangeLogComparator() {
    }

    @Override
    public int compare(String o1, String o2) {
        if (o1.contains(INIT) && !o2.contains(INIT)) {
            return -1;
        } else if (o2.contains(INIT) && !o1.contains(INIT)) {
            return 1;
        } else {
            return o1.replace("WEB-INF/classes/", "").compareTo(o2.replace("WEB-INF/classes/", ""));
        }
    }
}