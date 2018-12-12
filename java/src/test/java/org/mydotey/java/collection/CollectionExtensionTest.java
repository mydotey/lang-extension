package org.mydotey.java.collection;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.junit.Assert;
import org.junit.Test;
import org.mydotey.java.ObjectExtension;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class CollectionExtensionTest {

    @Test
    public void isEmpty() {
        Assert.assertTrue(CollectionExtension.isEmpty(ObjectExtension.<List<?>> NULL()));
        Assert.assertTrue(CollectionExtension.isEmpty(new ArrayList<>()));
        Assert.assertFalse(CollectionExtension.isEmpty(Arrays.asList(1)));

        Assert.assertTrue(CollectionExtension.isEmpty(ObjectExtension.<Object[]> NULL()));
        Assert.assertTrue(CollectionExtension.isEmpty(new Object[] {}));
        Assert.assertFalse(CollectionExtension.isEmpty(new Integer[] { 1 }));

        Assert.assertTrue(CollectionExtension.isEmpty(ObjectExtension.<Map<?, ?>> NULL()));
        Assert.assertTrue(CollectionExtension.isEmpty(new HashMap<>()));
        Map<Object, Object> map = new HashMap<>();
        map.put(new Object(), new Object());
        Assert.assertFalse(CollectionExtension.isEmpty(map));
    }

}
