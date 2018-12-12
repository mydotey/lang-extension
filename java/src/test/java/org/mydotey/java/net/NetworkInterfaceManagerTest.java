package org.mydotey.java.net;

import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class NetworkInterfaceManagerTest {

    @Test
    public void showInterfaceInfo() {
        System.out.printf("ip: %s\n", NetworkInterfaceManager.INSTANCE.localhostIP());
        System.out.printf("host: %s\n", NetworkInterfaceManager.INSTANCE.localhostName());
    }

}
