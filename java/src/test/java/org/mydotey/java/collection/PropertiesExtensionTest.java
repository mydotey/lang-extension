package org.mydotey.java.collection;

import java.util.Properties;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Jan 10, 2019
 */
public class PropertiesExtensionTest {

    @Test
    public void cloneTest() {
        Properties properties = null;
        Properties copy = PropertiesExtension.clone(properties);
        Assert.assertNull(copy);

        properties = new Properties();
        properties.put("k1", "v1");
        copy = PropertiesExtension.clone(properties);
        Assert.assertNotSame(properties, copy);
        Assert.assertEquals(properties, copy);

        copy = new Properties(properties);
        Assert.assertNotEquals(properties, copy);
    }

}
