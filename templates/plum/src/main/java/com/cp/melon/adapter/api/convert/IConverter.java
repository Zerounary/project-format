package com.cp.melon.adapter.api.convert;

import java.util.List;

/**
 * @Author sc
 * @Date 2022/10/28 10:15
 */
public interface IConverter<V,B> {

    public B vo2bo(V vo);

    public V bo2vo(B bo);

    public List<B> voList2boList(List<V> vos);

    public List<V> boList2voList(List<B> bos);
}
