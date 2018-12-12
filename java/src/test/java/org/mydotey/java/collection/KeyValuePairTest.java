package org.mydotey.java.collection;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class KeyValuePairTest {

    @Test
    public void object() {
        KeyValuePair<String, Integer> pair = new KeyValuePair<>();
        String expected = null;
        String actual = pair.toString();
        Assert.assertEquals(expected, actual);

        pair.setKey("k");
        expected = "k: ";
        actual = pair.toString();
        Assert.assertEquals(expected, actual);
        pair.setKey(null);

        pair.setValue(1);
        expected = ": 1";
        actual = pair.toString();
        Assert.assertEquals(expected, actual);
        pair.setValue(null);

        pair.setKey("k");
        pair.setValue(1);
        expected = "k: 1";
        actual = pair.toString();
        Assert.assertEquals(expected, actual);

        KeyValuePair<String, Integer> pair2 = new KeyValuePair<>("k", 1);
        expected = "k: 1";
        actual = pair.toString();
        Assert.assertEquals(expected, actual);
        Assert.assertEquals(pair, pair2);
        Assert.assertEquals(pair.hashCode(), pair2.hashCode());
    }

}
