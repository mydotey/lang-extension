package org.mydotey.java;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public class ObjectScope<T> {

    private T _default;

    private ThreadLocal<T> _current = new ThreadLocal<>();

    private CloseableScope _scope = new CloseableScope(this);

    public ObjectScope() {
        this(null);
    }

    public ObjectScope(T defaultValue) {
        _default = defaultValue;
    }

    public T current() {
        T current = _current.get();
        return current == null ? _default : current;
    }

    public CloseableScope use(T value) {
        _current.set(value);
        return _scope;
    }

    public static class CloseableScope implements AutoCloseable {

        private ObjectScope<?> _objectScope;

        public CloseableScope(ObjectScope<?> objectScope) {
            _objectScope = objectScope;
        }

        @Override
        public void close() {
            _objectScope._current.set(null);
        }

    }

}
