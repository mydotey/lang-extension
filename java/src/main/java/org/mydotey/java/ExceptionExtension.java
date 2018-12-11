package org.mydotey.java;

/**
 * @author koqizhao
 *
 * Dec 11, 2018
 */
public interface ExceptionExtension {

    static Throwable getRootCause(Throwable throwable) {
        ObjectExtension.requireNonNull(throwable, "throwable");

        while (throwable.getCause() != null)
            throwable = throwable.getCause();
        return throwable;
    }

}
