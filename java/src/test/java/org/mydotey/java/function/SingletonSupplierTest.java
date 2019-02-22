package org.mydotey.java.function;

import java.util.Comparator;
import java.util.concurrent.ConcurrentSkipListSet;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Supplier;

import org.junit.Assert;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

/**
 * @author koqizhao
 *
 * @date Nov 29, 2017
 */
public class SingletonSupplierTest {

    private int _concurrency = 50;
    private int _completeTime = 100;

    private static Comparator<Object> _objectComparator = new Comparator<Object>() {
        @Override
        public int compare(Object o1, Object o2) {
            if (o1 == o2)
                return 0;

            return o1.hashCode() > o2.hashCode() ? 1 : -1;
        }
    };

    @Rule
    public final ExpectedException expectedExceptionRule = ExpectedException.none();

    @Test
    public void test() {
        expectedExceptionRule.expect(IllegalStateException.class);
        SingletonSupplier<Object> supplier = new SingletonSupplier<>(() -> null);
        supplier.get();
    }

    @Test
    public void test2() {
        Object obj = new Object();
        test(() -> obj);

        test(() -> new Object());
    }

    private void test(Supplier<Object> getter) {
        getter = new SingletonSupplier<>(getter);
        boolean isSingleton = isSingletonGet(getter);
        Assert.assertTrue(isSingleton);
    }

    private boolean isSingletonGet(Supplier<Object> getter) {
        ConcurrentSkipListSet<Object> set = new ConcurrentSkipListSet<>(_objectComparator);
        AtomicBoolean canStart = new AtomicBoolean();
        AtomicInteger completed = new AtomicInteger();
        for (int i = 0; i < _concurrency; i++) {
            Thread thread = new Thread(new Runnable() {
                @Override
                public void run() {
                    while (true) {
                        if (canStart.get()) {
                            set.add(getter.get());
                            completed.incrementAndGet();
                            break;
                        }

                        Thread.yield();
                    }
                }
            });
            thread.setDaemon(true);
            thread.start();
        }

        canStart.set(true);

        try {
            Thread.sleep(_completeTime);
        } catch (Exception ex) {

        }
        return completed.get() == _concurrency && set.size() == 1;
    }

}
