package org.mydotey.java.net;

import java.net.Inet4Address;
import java.net.InetAddress;
import java.net.NetworkInterface;
import java.net.SocketException;
import java.net.UnknownHostException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import org.mydotey.java.StringExtension;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class NetworkInterfaceManager {

    private static Logger _logger = LoggerFactory.getLogger(NetworkInterfaceManager.class);

    public static final NetworkInterfaceManager INSTANCE = new NetworkInterfaceManager();

    private String _hostIP;
    private String _hostName;

    private NetworkInterfaceManager() {
        _hostIP = StringExtension.trim(System.getProperty("host.ip"));
        _hostName = StringExtension.trim(System.getProperty("host.name"));

        InetAddress localIP = null;
        try {
            localIP = getLocalIP();
        } catch (Throwable ex) {
            _logger.error("IPv4 address or local hostName auto find failed!", ex);
        }

        if (StringExtension.isBlank(_hostIP) && localIP != null)
            _hostIP = localIP.getHostAddress();

        if (StringExtension.isBlank(_hostName)) {
            try {
                _hostName = InetAddress.getLocalHost().getHostName();
            } catch (Throwable ex) {
                _logger.error("Cannot get hostName from InetAddress.getLocalHost().", ex);
            }
        }
        if (StringExtension.isBlank(_hostName) && localIP != null)
            _hostName = localIP.getHostName();
        if (StringExtension.isBlank(_hostName))
            _hostName = _hostIP;

        _logger.info("{} use: { hostIP: {}, hostName: {} }", NetworkInterfaceManager.class.getSimpleName(), _hostIP,
            _hostName);
    }

    public String hostIP() {
        if (_hostIP == null)
            throw new RuntimeException("Cannot find local ipv4 address!");

        return _hostIP;
    }

    public String hostName() {
        if (_hostName == null)
            throw new RuntimeException("Cannot find local hostName!");

        return _hostName;
    }

    private InetAddress getLocalIP() throws SocketException, UnknownHostException {
        List<NetworkInterface> nis = Collections.list(NetworkInterface.getNetworkInterfaces());
        List<Inet4Address> ipV4Addresses = new ArrayList<>();
        try {
            for (NetworkInterface ni : nis) {
                if (!ni.isUp() || ni.isLoopback() || ni.isVirtual())
                    continue;

                List<InetAddress> list = Collections.list(ni.getInetAddresses());
                for (InetAddress address : list) {
                    if (address.isLoopbackAddress())
                        continue;

                    if (address instanceof Inet4Address)
                        ipV4Addresses.add((Inet4Address) address);
                }
            }
        } catch (Throwable ex) {
            _logger.error("Get local ip failed", ex);
        }

        InetAddress address = getValidIPv4(ipV4Addresses);
        if (address == null)
            address = InetAddress.getLocalHost();
        return address;
    }

    private Inet4Address getValidIPv4(List<Inet4Address> addresses) {
        Inet4Address valid = null;

        int maxWeight = -1;
        for (Inet4Address address : addresses) {
            int weight = 0;

            if (!address.getHostName().equals(address.getHostAddress()))
                weight += 1;

            if (address.isLinkLocalAddress())
                weight += 4;

            if (address.isSiteLocalAddress())
                weight += 8;

            if (weight > maxWeight) {
                maxWeight = weight;
                valid = address;
            }
        }

        return valid;
    }

}
