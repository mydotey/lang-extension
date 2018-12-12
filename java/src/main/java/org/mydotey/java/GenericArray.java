package org.mydotey.java;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public class GenericArray<T> implements Iterable<T> {

    private List<T> _data;
    private int _length;

    public GenericArray(int length) {
        if (length < 0)
            throw new IllegalArgumentException("Array length cannot be less than 0.");

        _length = length;
        _data = new ArrayList<>(_length);
        for (int index = 0; index < _length; index++) {
            _data.add(null);
        }
    }

    public void set(int index, T element) {
        checkIndex(index);
        _data.set(index, element);
    }

    public T get(int index) {
        checkIndex(index);
        return _data.get(index);
    }

    public int length() {
        return _length;
    }

    public T[] toArray(T[] a) {
        return _data.toArray(a);
    }

    public List<T> toList() {
        return Collections.unmodifiableList(_data);
    }

    public void clear() {
        for (int index = 0; index < _length; index++) {
            _data.set(index, null);
        }
    }

    @Override
    public Iterator<T> iterator() {
        return _data.iterator();
    }

    private void checkIndex(int index) {
        if (index < 0 || index >= _length)
            throw new IndexOutOfBoundsException();
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((_data == null) ? 0 : _data.hashCode());
        result = prime * result + _length;
        return result;
    }

    @SuppressWarnings("rawtypes")
    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        GenericArray other = (GenericArray) obj;
        if (_data == null) {
            if (other._data != null)
                return false;
        } else if (!_data.equals(other._data))
            return false;
        if (_length != other._length)
            return false;
        return true;
    }

    @Override
    public String toString() {
        return _data.toString();
    }

}
