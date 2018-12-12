package org.mydotey.java;

import java.io.ByteArrayOutputStream;
import java.io.Closeable;
import java.io.IOException;

import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class ClosableExtensionTest {

    @Test
    public void close() {
        CloseableExtension.close(new ByteArrayOutputStream());
        CloseableExtension.close(new TestClosable());
    }

    private class TestClosable implements Closeable {

        @Override
        public void close() throws IOException {
            throw new IOException();
        }

    }

}
