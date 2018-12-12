package org.mydotey.java;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class ExceptionExtensionTest {

    @Test
    public void getRootCause() {
        Exception ex = new Exception();
        Assert.assertSame(ex, ExceptionExtension.getRootCause(ex));

        Exception ex2 = new Exception(ex);
        Assert.assertSame(ex, ExceptionExtension.getRootCause(ex2));
    }

}
