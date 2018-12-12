package org.mydotey.java;

import java.util.List;
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
public class GenericArrayTest {

    @Rule
    public ExpectedException thrown = ExpectedException.none();

    @Test
    public void length() {
        int length = 10;
        GenericArray<String> stringArray = new GenericArray<>(length);
        Assert.assertEquals(length, stringArray.length());

        length = 0;
        stringArray = new GenericArray<>(length);
        Assert.assertEquals(length, stringArray.length());
    }

    @Test
    public void length2() {
        thrown.expect(IllegalArgumentException.class);

        int length = -1;
        new GenericArray<>(length);
    }

    @Test
    public void getSet() {
        int length = 10;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(5, "test5");
        testData.put(9, "test9");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> {
            stringArray.set(i, d);
            Assert.assertEquals(d, stringArray.get(i));
        });

        Assert.assertNull(stringArray.get(1));
    }

    @Test
    public void getSet2() {
        thrown.expect(IndexOutOfBoundsException.class);

        int length = 10;
        GenericArray<String> stringArray = new GenericArray<>(length);
        stringArray.set(length, "test");
    }

    @Test
    public void getSet3() {
        thrown.expect(IndexOutOfBoundsException.class);

        int length = 10;
        GenericArray<String> stringArray = new GenericArray<>(length);
        stringArray.set(-1, "test");
    }

    @Test
    public void getSet4() {
        thrown.expect(IndexOutOfBoundsException.class);

        int length = 10;
        GenericArray<String> stringArray = new GenericArray<>(length);
        stringArray.get(length);
    }

    @Test
    public void getSet5() {
        thrown.expect(IndexOutOfBoundsException.class);

        int length = 10;
        GenericArray<String> stringArray = new GenericArray<>(length);
        stringArray.get(-1);
    }

    @Test
    public void toArray() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        String[] stringArray2 = new String[stringArray.length()];
        String[] stringArray3 = stringArray.toArray(stringArray2);
        Assert.assertEquals(stringArray.length(), stringArray2.length);
        Assert.assertSame(stringArray2, stringArray3);
        for (int i = 0; i < stringArray.length(); i++) {
            Assert.assertEquals(stringArray.get(i), stringArray2[i]);
        }

        stringArray2 = new String[stringArray.length() - 1];
        stringArray3 = stringArray.toArray(stringArray2);
        Assert.assertNotEquals(stringArray.length(), stringArray2.length);
        Assert.assertNotSame(stringArray2, stringArray3);
        for (int i = 0; i < stringArray.length(); i++) {
            Assert.assertEquals(stringArray.get(i), stringArray3[i]);
        }

        stringArray2 = new String[stringArray.length() + 1];
        stringArray3 = stringArray.toArray(stringArray2);
        Assert.assertNotEquals(stringArray.length(), stringArray2.length);
        Assert.assertSame(stringArray2, stringArray3);
        for (int i = 0; i < stringArray.length(); i++) {
            Assert.assertEquals(stringArray.get(i), stringArray2[i]);
        }
        Assert.assertNull(stringArray2[stringArray.length()]);
    }

    @Test
    public void toList() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        List<String> stringList = stringArray.toList();
        Assert.assertEquals(length, stringList.size());
        for (int i = 0; i < length; i++) {
            Assert.assertEquals(stringArray.get(i), stringList.get(i));
        }

        GenericArray<String> stringArray2 = new GenericArray<>(0);
        stringList = stringArray2.toList();
        Assert.assertTrue(stringList.isEmpty());
    }

    @Test
    public void toList2() {
        thrown.expect(UnsupportedOperationException.class);

        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        List<String> stringList = stringArray.toList();
        stringList.add("test2");
    }

    @Test
    public void toList3() {
        thrown.expect(UnsupportedOperationException.class);

        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        List<String> stringList = stringArray.toList();
        stringList.remove(0);
    }

    @Test
    public void toList4() {
        thrown.expect(UnsupportedOperationException.class);

        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        List<String> stringList = stringArray.toList();
        stringList.set(0, "test2");
    }

    @Test
    public void clear() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        stringArray.clear();
        Assert.assertEquals(length, stringArray.length());
        for (int i = 0; i < length; i++)
            Assert.assertNull(stringArray.get(i));
    }

    @Test
    public void iterable() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        testData.forEach((i, d) -> stringArray.set(i, d));
        for (String d : stringArray)
            System.out.println(d);
    }

    @Test
    public void equals() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");

        GenericArray<String> stringArray = new GenericArray<>(length);
        GenericArray<String> stringArray2 = new GenericArray<>(length);
        Assert.assertNotSame(stringArray, stringArray2);
        Assert.assertEquals(stringArray, stringArray2);
        Assert.assertEquals(stringArray.hashCode(), stringArray2.hashCode());

        testData.forEach((i, d) -> stringArray.set(i, d));
        Assert.assertNotEquals(stringArray, stringArray2);
        Assert.assertNotEquals(stringArray.hashCode(), stringArray2.hashCode());

        testData.forEach((i, d) -> stringArray2.set(i, d));
        Assert.assertEquals(stringArray, stringArray2);
        Assert.assertEquals(stringArray.hashCode(), stringArray2.hashCode());
    }

    @Test
    public void toStringTest() {
        int length = 2;
        Map<Integer, String> testData = new HashMap<>();
        testData.put(0, "test");
        testData.put(1, "test1");
        GenericArray<String> stringArray = new GenericArray<>(length);
        List<String> stringList = stringArray.toList();
        String s = stringArray.toString();
        System.out.println(s);
        Assert.assertEquals(stringList.toString(), s);

        testData.forEach((i, d) -> stringArray.set(i, d));
        String s2 = stringArray.toString();
        System.out.println(s2);
        Assert.assertNotEquals(s, s2);
        Assert.assertEquals(stringList.toString(), s2);
    }

}
