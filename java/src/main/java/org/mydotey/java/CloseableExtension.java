package org.mydotey.java;

import org.slf4j.LoggerFactory;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public interface CloseableExtension {

    static void close(AutoCloseable closeable) {
        try {
            if (closeable == null)
                return;

            closeable.close();
        } catch (Exception e) {
            LoggerFactory.getLogger(CloseableExtension.class).error("Close closeable failed", e);
        }
    }

}
