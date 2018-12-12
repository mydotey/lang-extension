package org.mydotey.java;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class BooleanExtensionTest {

    @Test
    public void isTrue() {
        Assert.assertTrue(BooleanExtension.isTrue(Boolean.TRUE));
        Assert.assertFalse(BooleanExtension.isTrue(Boolean.FALSE));
        Assert.assertFalse(BooleanExtension.isTrue(null));
    }

    @Test
    public void isFalse() {
        Assert.assertTrue(BooleanExtension.isFalse(Boolean.FALSE));
        Assert.assertFalse(BooleanExtension.isFalse(Boolean.TRUE));
        Assert.assertFalse(BooleanExtension.isFalse(null));
    }

}
