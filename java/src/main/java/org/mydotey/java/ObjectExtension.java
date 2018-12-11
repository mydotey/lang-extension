package org.mydotey.java;

import java.util.Collection;
import java.util.Map;

import org.mydotey.java.collection.CollectionExtension;

/**
 * @author koqizhao
 *
 * Dec 11, 2018
 */
public interface ObjectExtension {

    static <T> T NULL() {
        return null;
    }

    static void requireNonNull(Object obj, String name) {
        if (obj == null)
            throw new IllegalArgumentException(name + " is null");
    }

    static void requireNonEmpty(Collection<?> obj, String name) {
        if (CollectionExtension.isEmpty(obj))
            throw new IllegalArgumentException(name + " is empty");
    }

    static <T> void requireNonEmpty(T[] obj, String name) {
        if (CollectionExtension.isEmpty(obj))
            throw new IllegalArgumentException(name + " is empty");
    }

    static void requireNonEmpty(Map<?, ?> obj, String name) {
        if (CollectionExtension.isEmpty(obj))
            throw new IllegalArgumentException(name + " is empty");
    }

    static void requireNonEmpty(String obj, String name) {
        if (StringExtension.isEmpty(obj))
            throw new IllegalArgumentException(name + " is empty");
    }

    static void requireNonBlank(String obj, String name) {
        if (StringExtension.isBlank(obj))
            throw new IllegalArgumentException(name + " is blank");
    }

}
