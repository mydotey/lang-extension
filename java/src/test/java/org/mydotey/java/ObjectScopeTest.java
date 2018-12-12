package org.mydotey.java;

import java.util.Arrays;
import java.util.Objects;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class ObjectScopeTest {

    @Test
    public void use() throws Exception {
        ObjectScope<String> objectScope = new ObjectScope<>();
        Assert.assertNull(objectScope.current());

        String obj = "test";
        try (AutoCloseable closeable = objectScope.use(obj)) {
            Assert.assertEquals(obj, objectScope.current());
        }
        Assert.assertNull(objectScope.current());

        String defaultValue = "default";
        objectScope = new ObjectScope<>(defaultValue);
        Assert.assertEquals(defaultValue, objectScope.current());

        try (AutoCloseable closeable = objectScope.use(obj)) {
            Assert.assertEquals(obj, objectScope.current());
        }
        Assert.assertEquals(defaultValue, objectScope.current());
    }

    @Test
    public void use2() throws InterruptedException {
        ObjectScope<String> objectScope = new ObjectScope<>();
        int concurrency = 100;
        Thread[] threads = new Thread[concurrency];
        AtomicBoolean failed = new AtomicBoolean();
        CountDownLatch latch = new CountDownLatch(concurrency);
        for (int i = 0; i < concurrency; i++) {
            int j = i;
            threads[j] = new Thread(() -> {
                System.out.printf("thread %s start\n", j);
                String current = objectScope.current();
                if (current != null) {
                    failed.set(true);
                    System.out.printf("expected: %s, current: %s\n", null, current);
                }

                String obj = "test" + j;
                try (AutoCloseable closeable = objectScope.use(obj)) {
                    current = objectScope.current();
                    if (!Objects.equals(obj, current)) {
                        failed.set(true);
                        System.out.printf("expected: %s, current: %s\n", obj, current);
                    }
                } catch (Exception e) {
                    failed.set(true);
                    e.printStackTrace();
                }

                current = objectScope.current();
                if (current != null) {
                    failed.set(true);
                    System.out.printf("expected: %s, current: %s\n", null, current);
                }

                latch.countDown();
                System.out.printf("thread %s end\n", j);
            });
            threads[i].setDaemon(true);
        }

        Arrays.stream(threads).forEach(t -> t.start());

        latch.await(10, TimeUnit.SECONDS);
        Assert.assertFalse(failed.get());
    }

}
