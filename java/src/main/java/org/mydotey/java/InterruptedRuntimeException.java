package org.mydotey.java;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class InterruptedRuntimeException extends RuntimeException {

    private static final long serialVersionUID = 1L;

    public InterruptedRuntimeException(InterruptedException cause) {
        super(cause);
    }

}
