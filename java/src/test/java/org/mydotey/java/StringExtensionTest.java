package org.mydotey.java;

import java.util.Arrays;
import java.util.Collection;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class StringExtensionTest {

    @Test
    public void NULL() {
        Assert.assertEquals("null", StringExtension.NULL);
    }

    @Test
    public void isEmpty() {
        Assert.assertTrue(StringExtension.isEmpty(null));
        Assert.assertTrue(StringExtension.isEmpty(""));
        Assert.assertFalse(StringExtension.isEmpty(" "));
    }

    @Test
    public void isBlank() {
        Assert.assertTrue(StringExtension.isBlank(null));
        Assert.assertTrue(StringExtension.isBlank(""));
        Assert.assertTrue(StringExtension.isBlank(" "));
        Assert.assertFalse(StringExtension.isBlank("test"));
    }

    @Test
    public void nullToEmpty() {
        Assert.assertEquals(StringExtension.EMPTY, StringExtension.nullToEmpty(null));
        Assert.assertEquals(StringExtension.EMPTY, StringExtension.nullToEmpty(StringExtension.EMPTY));
        Assert.assertEquals("test", StringExtension.nullToEmpty("test"));
    }

    @Test
    public void toStringTest() {
        Object data = null;
        Assert.assertEquals(data, StringExtension.toString(data));

        data = new Object();
        Assert.assertEquals(data.toString(), StringExtension.toString(data));

        data = "test";
        Assert.assertSame(data.toString(), StringExtension.toString(data));
    }

    @Test
    public void toLowerCase() {
        String data = null;
        Assert.assertEquals(data, StringExtension.toLowerCase(data));

        data = "";
        Assert.assertEquals(data.toLowerCase(), StringExtension.toLowerCase(data));

        data = "ok";
        Assert.assertEquals(data.toLowerCase(), StringExtension.toLowerCase(data));

        data = "oK";
        Assert.assertEquals(data.toLowerCase(), StringExtension.toLowerCase(data));
    }

    @Test
    public void toUpperCase() {
        String data = null;
        Assert.assertEquals(data, StringExtension.toUpperCase(data));

        data = "";
        Assert.assertEquals(data.toUpperCase(), StringExtension.toUpperCase(data));

        data = "OK";
        Assert.assertEquals(data.toUpperCase(), StringExtension.toUpperCase(data));

        data = "oK";
        Assert.assertEquals(data.toUpperCase(), StringExtension.toUpperCase(data));
    }

    @Test
    public void intern() {
        String data = null;
        Assert.assertEquals(data, StringExtension.intern(data));

        data = "";
        Assert.assertEquals(data.intern(), StringExtension.intern(data));

        data = "oK";
        Assert.assertEquals(data.intern(), StringExtension.intern(data));
    }

    @Test
    public void equalsIgnoreCase() {
        String data = null;
        String data2 = null;
        Assert.assertEquals(true, StringExtension.equalsIgnoreCase(data, data2));

        data = null;
        data2 = "test";
        Assert.assertEquals(false, StringExtension.equalsIgnoreCase(data, data2));

        data = "test";
        data2 = null;
        Assert.assertEquals(data.equalsIgnoreCase(data2), StringExtension.equalsIgnoreCase(data, data2));

        data = "test";
        data2 = "test";
        Assert.assertEquals(data.equals(data2), StringExtension.equalsIgnoreCase(data, data2));

        data = "test";
        data2 = "test2";
        Assert.assertEquals(data.equals(data2), StringExtension.equalsIgnoreCase(data, data2));
    }

    @Test
    public void trim() {
        String data = null;
        char[] chars = new char[] {};
        String expected = data;
        String actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] {};
        expected = data;
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 'e' };
        expected = data;
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't' };
        expected = "es";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test2";
        chars = new char[] { 't' };
        expected = "est2";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "2test";
        chars = new char[] { 't' };
        expected = "2tes";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 'e', 't' };
        expected = "s";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e' };
        expected = "s";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', 't' };
        expected = "es";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', '2' };
        expected = "test";
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's', '1', '2' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trim(data, chars);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void trimStart() {
        String data = null;
        char[] chars = new char[] {};
        String expected = data;
        String actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] {};
        expected = data;
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 'e' };
        expected = data;
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't' };
        expected = "est";
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "2test";
        chars = new char[] { 't' };
        expected = "2test";
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 'e', 't' };
        expected = "st";
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', 't' };
        expected = "est";
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', '2' };
        expected = "test";
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's', '1', '2' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trimStart(data, chars);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void trimEnd() {
        String data = null;
        char[] chars = new char[] {};
        String expected = data;
        String actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] {};
        expected = data;
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 'e' };
        expected = data;
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't' };
        expected = "tes";
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test2";
        chars = new char[] { 't' };
        expected = data;
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 's', 't' };
        expected = "te";
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', 't' };
        expected = "tes";
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { '1', '2' };
        expected = "test";
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);

        data = "test";
        chars = new char[] { 't', 'e', 's', '1', '2' };
        expected = StringExtension.EMPTY;
        actual = StringExtension.trimEnd(data, chars);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void trimString() {
        String data = null;
        String s = StringExtension.EMPTY;
        String expected = data;
        String actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = StringExtension.EMPTY;
        expected = data;
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "e";
        expected = data;
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "t";
        expected = "es";
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test2";
        s = "t";
        expected = "est2";
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "2test";
        s = "t";
        expected = "2tes";
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "te";
        expected = "st";
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "123";
        expected = "test";
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "test";
        expected = StringExtension.EMPTY;
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "testok";
        expected = data;
        actual = StringExtension.trim(data, s);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void trimStartString() {
        String data = null;
        String s = StringExtension.EMPTY;
        String expected = data;
        String actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = StringExtension.EMPTY;
        expected = data;
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "e";
        expected = data;
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "t";
        expected = "est";
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "2test";
        s = "t";
        expected = "2test";
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "te";
        expected = "st";
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "12";
        expected = "test";
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "tes";
        expected = "t";
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "test12";
        expected = data;
        actual = StringExtension.trimStart(data, s);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void trimEndString() {
        String data = null;
        String s = StringExtension.EMPTY;
        String expected = data;
        String actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = StringExtension.EMPTY;
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "e";
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "t";
        expected = "tes";
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test2";
        s = "t";
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "st";
        expected = "te";
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "1t";
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "12";
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "est";
        expected = "t";
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);

        data = "test";
        s = "12est";
        expected = data;
        actual = StringExtension.trimEnd(data, s);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void join() {
        Integer[] array = null;
        char c = '.';
        String expected = null;
        String actual = StringExtension.join(array, c);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1 };
        c = '.';
        expected = "1";
        actual = StringExtension.join(array, c);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1, 2, 3 };
        c = '.';
        expected = "1.2.3";
        actual = StringExtension.join(array, c);
        Assert.assertEquals(expected, actual);

        array = null;
        String s = ".";
        expected = null;
        actual = StringExtension.join(array, s);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1 };
        s = ".";
        expected = "1";
        actual = StringExtension.join(array, s);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1, 2, 3 };
        s = ".";
        expected = "1.2.3";
        actual = StringExtension.join(array, s);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1, 2, 3 };
        s = StringExtension.EMPTY;
        expected = "123";
        actual = StringExtension.join(array, s);
        Assert.assertEquals(expected, actual);

        array = new Integer[] { 1, 2, 3 };
        s = "..";
        expected = "1..2..3";
        actual = StringExtension.join(array, s);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void join2() {
        Collection<?> collection = null;
        char c = '.';
        String expected = null;
        String actual = StringExtension.join(collection, c);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1);
        c = '.';
        expected = "1";
        actual = StringExtension.join(collection, c);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1, 2, 3);
        c = '.';
        expected = "1.2.3";
        actual = StringExtension.join(collection, c);
        Assert.assertEquals(expected, actual);

        collection = null;
        String s = ".";
        expected = null;
        actual = StringExtension.join(collection, s);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1);
        s = ".";
        expected = "1";
        actual = StringExtension.join(collection, s);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1, 2, 3);
        s = ".";
        expected = "1.2.3";
        actual = StringExtension.join(collection, s);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1, 2, 3);
        s = StringExtension.EMPTY;
        expected = "123";
        actual = StringExtension.join(collection, s);
        Assert.assertEquals(expected, actual);

        collection = Arrays.asList(1, 2, 3);
        s = "..";
        expected = "1..2..3";
        actual = StringExtension.join(collection, s);
        Assert.assertEquals(expected, actual);
    }

}
