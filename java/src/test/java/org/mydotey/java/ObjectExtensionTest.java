package org.mydotey.java;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.junit.Assert;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class ObjectExtensionTest {

    @Rule
    public ExpectedException thrown = ExpectedException.none();

    @Test
    public void NULL() {
        Object obj = ObjectExtension.NULL();
        Assert.assertNull(obj);

        Integer obj2 = ObjectExtension.NULL();
        Assert.assertNull(obj2);

        String obj3 = ObjectExtension.NULL();
        Assert.assertNull(obj3);
    }

    @Test
    public void requireNonNull() {
        Object obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null");
        ObjectExtension.requireNonNull(obj, name);
    }

    @Test
    public void requireNonNull2() {
        Object obj = new Object();
        String name = "arg";
        ObjectExtension.requireNonNull(obj, name);
    }

    @Test
    public void requireNonEmpty() {
        String obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty2() {
        String obj = "";
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty3() {
        String obj = "  ";
        String name = "arg";
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonBlank() {
        String obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is blank");
        ObjectExtension.requireNonBlank(obj, name);
    }

    @Test
    public void requireNonBlank2() {
        String obj = "";
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is blank");
        ObjectExtension.requireNonBlank(obj, name);
    }

    @Test
    public void requireNonBlank3() {
        String obj = "  ";
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is blank");
        ObjectExtension.requireNonBlank(obj, name);
    }

    @Test
    public void requireNonBlank4() {
        String obj = "test";
        String name = "arg";
        ObjectExtension.requireNonBlank(obj, name);
    }

    @Test
    public void requireNonEmpty4() {
        Collection<?> obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty5() {
        Collection<?> obj = new ArrayList<>();
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty6() {
        Collection<?> obj = Arrays.asList(new Object());
        String name = "arg";
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty7() {
        Object[] obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty8() {
        Object[] obj = new Object[] {};
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty9() {
        Object[] obj = new Object[] { new Object() };
        String name = "arg";
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty10() {
        Map<?, ?> obj = null;
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty11() {
        Map<?, ?> obj = new HashMap<>();
        String name = "arg";
        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is empty");
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonEmpty12() {
        Map<String, String> obj = new HashMap<>();
        obj.put("", "");
        String name = "arg";
        ObjectExtension.requireNonEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty() {
        Object obj = new Object();
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(obj, name);

        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null or empty");
        obj = null;
        ObjectExtension.requireNonNullOrEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty2() {
        String obj = "test";
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(obj, name);

        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null or empty");
        obj = "  ";
        ObjectExtension.requireNonNullOrEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty3() {
        List<?> obj = Arrays.asList(1, 2);
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(obj, name);

        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null or empty");
        obj = new ArrayList<>();
        ObjectExtension.requireNonNullOrEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty4() {
        Map<?, ?> obj = Collections.singletonMap("k", "v");
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(obj, name);

        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null or empty");
        obj = new HashMap<>();
        ObjectExtension.requireNonNullOrEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty5() {
        Object[] obj = new Object[] { 1, 2 };
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(obj, name);

        thrown.expect(IllegalArgumentException.class);
        thrown.expectMessage(name + " is null or empty");
        obj = new Object[] {};
        ObjectExtension.requireNonNullOrEmpty(obj, name);
    }

    @Test
    public void requireNonNullOrEmpty6() {
        String name = "arg";
        ObjectExtension.requireNonNullOrEmpty(0, name);
        ObjectExtension.requireNonNullOrEmpty(0.0, name);
        ObjectExtension.requireNonNullOrEmpty(false, name);
        ObjectExtension.requireNonNullOrEmpty(1, name);
        ObjectExtension.requireNonNullOrEmpty(new Object(), name);
    }

    @Test
    public void isNullOrEmpty() {
        Assert.assertTrue(ObjectExtension.isNullOrEmpty(null));
        Assert.assertTrue(ObjectExtension.isNullOrEmpty(""));
        Assert.assertTrue(ObjectExtension.isNullOrEmpty("  "));
        Assert.assertTrue(ObjectExtension.isNullOrEmpty(new ArrayList<>()));
        Assert.assertTrue(ObjectExtension.isNullOrEmpty(new HashMap<>()));
        Assert.assertTrue(ObjectExtension.isNullOrEmpty(new Object[0]));
        Assert.assertFalse(ObjectExtension.isNullOrEmpty(0));
        Assert.assertFalse(ObjectExtension.isNullOrEmpty(0.0));
        Assert.assertFalse(ObjectExtension.isNullOrEmpty(false));
        Assert.assertFalse(ObjectExtension.isNullOrEmpty(new Object()));
    }

}
