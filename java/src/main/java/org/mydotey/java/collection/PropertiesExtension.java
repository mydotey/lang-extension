package org.mydotey.java.collection;

import java.util.Properties;

/**
 * @author koqizhao
 *
 * Jan 10, 2019
 */
public interface PropertiesExtension {

    static Properties clone(Properties properties) {
        if (properties == null)
            return null;

        Properties copy = new Properties();
        copy.putAll(properties);
        return copy;
    }

}
