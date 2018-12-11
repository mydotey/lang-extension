package org.mydotey.java;

/**
 * Created by Qiang Zhao on 10/05/2016.
 */
public interface BooleanExtension {

    static boolean isTrue(Boolean value) {
        return Boolean.TRUE.equals(value);
    }

    static boolean isFalse(Boolean value) {
        return Boolean.FALSE.equals(value);
    }

}
