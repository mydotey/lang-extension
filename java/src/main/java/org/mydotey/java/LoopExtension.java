package org.mydotey.java;

import java.util.function.Supplier;

import org.slf4j.LoggerFactory;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public interface LoopExtension {

    static final int DEFAULT_SLEEP_NANOS_IN_TIGHT_LOOP = 1 * 1000;

    static <V> V executeWithoutTightLoop(Supplier<V> func) throws InterruptedException {
        return executeWithoutTightLoop(func, 0, DEFAULT_SLEEP_NANOS_IN_TIGHT_LOOP);
    }

    static <V> V executeWithoutTightLoop(Supplier<V> func, int ms, int nanos) throws InterruptedException {
        ObjectExtension.requireNonNull(func, "func");

        long startTime = System.currentTimeMillis();
        try {
            return func.get();
        } finally {
            if (System.currentTimeMillis() - startTime <= 0)
                preventTightLoop(ms, nanos);
        }
    }

    static void executeWithoutTightLoop(Runnable action) throws InterruptedException {
        executeWithoutTightLoop(action, 0, DEFAULT_SLEEP_NANOS_IN_TIGHT_LOOP);
    }

    static void executeWithoutTightLoop(Runnable action, int ms, int nanos) throws InterruptedException {
        ObjectExtension.requireNonNull(action, "action");

        long startTime = System.currentTimeMillis();
        try {
            action.run();
        } finally {
            if (System.currentTimeMillis() - startTime <= 0)
                preventTightLoop(ms, nanos);
        }
    }

    static void preventTightLoop() throws InterruptedException {
        preventTightLoop(0, DEFAULT_SLEEP_NANOS_IN_TIGHT_LOOP);
    }

    static void preventTightLoop(int ms, int nanos) throws InterruptedException {
        LoggerFactory.getLogger(LoopExtension.class).info("Sleep {} ms & {} nanos to prevent tight loop.", ms, nanos);
        Thread.sleep(ms, nanos);
    }

}
