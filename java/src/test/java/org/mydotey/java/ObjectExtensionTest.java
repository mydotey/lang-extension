package org.mydotey.java;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.HashMap;
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

}
