package org.mydotey.java.collection;

import java.util.Arrays;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class MultiWriteBatchReadListTest {

    @Test
    public void writeReadTest() {
        MultiWriteBatchReadList<String> list = new MultiWriteBatchReadList<>();
        Assert.assertEquals(0, list.size());

        list.add("t1");
        Assert.assertEquals(1, list.size());

        list.add("t2");
        Assert.assertEquals(2, list.size());

        List<String> all = list.getAll();
        Assert.assertEquals(2, all.size());
        Assert.assertEquals("t1", all.get(0));
        Assert.assertEquals("t2", all.get(1));

        list = new MultiWriteBatchReadList<>(10);
        Assert.assertEquals(0, list.size());

        all = list.getAll();
        Assert.assertEquals(0, all.size());
    }

    @Test
    public void writeReadTest2() throws InterruptedException {
        MultiWriteBatchReadList<String> list = new MultiWriteBatchReadList<>();
        int concurrency = 100;
        int count = 100;
        Thread[] threads = new Thread[concurrency];
        CountDownLatch latch = new CountDownLatch(concurrency);
        for (int i = 0; i < concurrency; i++) {
            int j = i;
            threads[j] = new Thread(() -> {
                System.out.printf("thread %s start\n", j);
                for (int k = 0; k < count; k++)
                    list.add(String.valueOf(j) + String.valueOf(k));
                latch.countDown();
                System.out.printf("thread %s end\n", j);
            });
            threads[j].setDaemon(true);
        }
        Arrays.stream(threads).forEach(t -> t.start());

        latch.await(10, TimeUnit.SECONDS);
        Assert.assertEquals(concurrency * count, list.size());
    }

}
