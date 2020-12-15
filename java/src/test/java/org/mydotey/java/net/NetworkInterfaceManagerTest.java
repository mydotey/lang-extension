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
        //System.setProperty("host.ip", "192.168.56.11");
        //System.setProperty("host.name", "my-mac");
        System.out.printf("ip: %s\n", NetworkInterfaceManager.INSTANCE.hostIP());
        System.out.printf("host: %s\n", NetworkInterfaceManager.INSTANCE.hostName());
    }

}
