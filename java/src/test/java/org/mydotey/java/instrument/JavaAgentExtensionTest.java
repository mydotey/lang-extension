package org.mydotey.java.instrument;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class JavaAgentExtensionTest {

    public static void agentMain() {

    }

    public static void agentMain(String args[]) {

    }

    @Test
    public void loadAgent() {
        System.out.println("will fail, only show usage");

        boolean success = JavaAgentExtension.loadAgent(JavaAgentExtensionTest.class);
        Assert.assertFalse(success);

        success = JavaAgentExtension.loadAgent(JavaAgentExtensionTest.class, "arg arg2");
        Assert.assertFalse(success);

        success = JavaAgentExtension.loadAgent("some.jar");
        Assert.assertFalse(success);

        success = JavaAgentExtension.loadAgent("some.jar", "arg arg2");
        Assert.assertFalse(success);
    }

}
