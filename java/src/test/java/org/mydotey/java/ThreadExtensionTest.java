package org.mydotey.java;

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
public class ThreadExtensionTest {

    @Test
    public void sleep() {
        long before = System.nanoTime();
        ThreadExtension.sleep(10);
        long after = System.nanoTime();
        long elipsed = after - before;
        System.out.println("elipsed: " + elipsed);
        Assert.assertTrue(elipsed >= 10 * 1000 * 1000);

        before = System.nanoTime();
        ThreadExtension.sleep(0, 1000);
        after = System.nanoTime();
        elipsed = after - before;
        System.out.println("elipsed: " + elipsed);
        Assert.assertTrue(elipsed >= 1000);

        before = System.nanoTime();
        ThreadExtension.sleep(10, 1000);
        after = System.nanoTime();
        elipsed = after - before;
        System.out.println("elipsed: " + elipsed);
        Assert.assertTrue(elipsed >= 10 * 1000 * 1000 + 1000);
    }

    @Test
    public void sleep2() throws InterruptedException {
        CountDownLatch latch = new CountDownLatch(1);
        AtomicBoolean interupted = new AtomicBoolean();
        Thread thread = new Thread(() -> {
            System.out.println("sleep start");
            try {
                ThreadExtension.sleep(100);
            } catch (InterruptedRuntimeException ex) {
                interupted.set(true);
            }

            latch.countDown();
            System.out.println("sleep end");
        });
        thread.setDaemon(true);
        Assert.assertEquals(Thread.State.NEW, thread.getState());

        thread.start();
        Thread.sleep(10);
        Assert.assertEquals(Thread.State.TIMED_WAITING, thread.getState());
        Assert.assertFalse(interupted.get());

        thread.interrupt();
        latch.await(1, TimeUnit.SECONDS);
        Thread.sleep(10);
        Assert.assertEquals(Thread.State.TERMINATED, thread.getState());
        Assert.assertTrue(interupted.get());
    }

}
