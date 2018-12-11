package org.mydotey.java.collection;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

import org.mydotey.java.ObjectExtension;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public class MultiWriteBatchReadList<V> {

    private volatile AtomicInteger _index = new AtomicInteger();

    private volatile ConcurrentHashMap<Integer, V> _data;

    public MultiWriteBatchReadList() {
        this(0);
    }

    public MultiWriteBatchReadList(int initialCapacity) {
        _data = new ConcurrentHashMap<>(initialCapacity);
    }

    public void add(V value) {
        ObjectExtension.requireNonNull(value, "value");
        _data.put(_index.getAndIncrement(), value);
    }

    public int size() {
        return _index.get();
    }

    public List<V> getAll() {
        List<V> values = new ArrayList<>();

        for (int i = 0; i < size(); i++) {
            V item = _data.get(i);
            if (item == null)
                break;

            values.add(item);
        }

        return values;
    }

}
