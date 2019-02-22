package org.mydotey.java.function;

import java.util.Objects;
import java.util.function.Supplier;

/**
 * @author koqizhao
 *
 * Feb 1, 2018
 */
public class SingletonSupplier<T> implements Supplier<T> {

    private volatile T _value;
    private Supplier<T> _supplier;

    public SingletonSupplier(Supplier<T> supplier) {
        Objects.requireNonNull(supplier, "supplier");
        _supplier = supplier;
    }

    @Override
    public T get() {
        if (_value == null) {
            synchronized (this) {
                if (_value == null) {
                    _value = _supplier.get();

                    if (_value == null)
                        throw new IllegalStateException(
                                "SingletonSupplier got null from the source supplier: " + _supplier);

                    _supplier = null;
                }
            }
        }

        return _value;
    }

}
