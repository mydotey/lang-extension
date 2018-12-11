package org.mydotey.java;

import org.slf4j.LoggerFactory;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public interface ThreadExtension {

    static void sleep(int ms) {
        sleep(ms, 0);
    }

    static void sleep(int ms, int nanos) {
        try {
            Thread.sleep(ms, nanos);
        } catch (InterruptedException ex) {
            LoggerFactory.getLogger(ThreadExtension.class).warn("Thread sleep interrupted.", ex);
        }
    }

}
