package org.mydotey.java.instrument;

import java.io.File;
import java.lang.management.ManagementFactory;
import java.lang.management.RuntimeMXBean;
import java.lang.reflect.Method;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Paths;
import java.security.CodeSource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * @author koqizhao
 *
 * Sep 6, 2018
 */
public class JavaAgentExtension {

    private static Logger _logger = LoggerFactory.getLogger(JavaAgentExtension.class);

    private static final String VM_CLASS_NAME = "com.sun.tools.attach.VirtualMachine";

    private static ClassLoader _toolsJarClassLoader;
    private static Class<?> _vmClazz;
    private static Method _attachMethod;
    private static Method _loadAgentMethod;
    private static Method _loadAgentMethod2;
    private static Method _detachMethod;

    private static String _processID;

    private static volatile Boolean _initSuccess;

    private JavaAgentExtension() {

    }

    public static boolean loadAgent(Class<?> agentClass) {
        return loadAgent(agentClass, null);
    }

    public static boolean loadAgent(Class<?> agentClass, String agentArgs) {
        String agentJar;
        try {
            CodeSource src = agentClass.getProtectionDomain().getCodeSource();
            agentJar = Paths.get(src.getLocation().toURI()).toString();
        } catch (Exception e) {
            _logger.error("failed to get agent jar for " + agentClass, e);
            return false;
        }

        _logger.info("got CodeSource {} for {}", agentJar, agentClass);
        if (!agentJar.endsWith(".jar") && !agentJar.endsWith(".war")) {
            _logger.error("CodeSource {} is not a .jar or .war java package", agentJar);
            return false;
        }
        return loadAgent(agentJar, agentArgs);
    }

    public static boolean loadAgent(String agentJar) {
        return loadAgent(agentJar, null);
    }

    public static boolean loadAgent(String agentJar, String agentArgs) {
        tryInit();

        if (_initSuccess == Boolean.FALSE)
            return false;

        try {
            Object vm = _attachMethod.invoke(null, new Object[] { _processID });
            if (agentArgs == null)
                _loadAgentMethod.invoke(vm, new Object[] { agentJar });
            else
                _loadAgentMethod2.invoke(vm, new Object[] { agentJar, agentArgs });
            _detachMethod.invoke(vm, new Object[0]);
            _logger.info("load java agent success: {}", agentJar);
            return true;
        } catch (Exception e) {
            _logger.error("load java agent failed", e);
            return false;
        }
    }

    private static void tryInit() {
        if (_initSuccess != null)
            return;

        synchronized (JavaAgentExtension.class) {
            if (_initSuccess != null)
                return;

            try {
                if (!initVMClass() || !initVMMethods()) {
                    _initSuccess = Boolean.FALSE;
                    _logger.error("init failed");
                    return;
                }

                initProcessID();
                _initSuccess = Boolean.TRUE;
                _logger.info("init success");
            } catch (Exception e) {
                _initSuccess = Boolean.FALSE;
                _logger.error("init failed", e);
            }
        }
    }

    private static String getToolsJarPath() {
        String javaHome = System.getProperty("java.home");
        String toolsJarPath = Paths.get(javaHome, "..", "lib/tools.jar").toAbsolutePath().normalize().toString();
        File file = new File(toolsJarPath);
        if (file.exists())
            return toolsJarPath;

        javaHome = System.getenv("JAVA_HOME");
        if (javaHome == null || javaHome.isEmpty())
            return null;

        toolsJarPath = Paths.get(javaHome, "lib/tools.jar").toAbsolutePath().normalize().toString();
        file = new File(toolsJarPath);
        return file.exists() ? toolsJarPath : null;
    }

    private static boolean initVMClass() {
        try {
            _vmClazz = Class.forName(VM_CLASS_NAME);
            _logger.info("loaded tools VM class from class path");
            return true;
        } catch (Exception e) {
            _logger.info("no tools VM class in class path");
        }

        String toolsJarPath = getToolsJarPath();
        if (toolsJarPath == null) {
            _logger.error("no tools jar found and cannot load tools VM class");
            return false;
        }

        _logger.info("found tools jar: {}", toolsJarPath);
        URL jarURl;
        try {
            jarURl = new File(toolsJarPath).toURI().toURL();
            _toolsJarClassLoader = new URLClassLoader(new URL[] { jarURl });
            _vmClazz = _toolsJarClassLoader.loadClass(VM_CLASS_NAME);
            _logger.info("loaded tools VM class from tools jar");
            return true;
        } catch (Exception e) {
            _logger.error("failed to load VM class from tools jar", e);
            return false;
        }
    }

    private static boolean initVMMethods() {
        try {
            _attachMethod = _vmClazz.getMethod("attach", new Class[] { String.class });
            _loadAgentMethod = _vmClazz.getMethod("loadAgent", new Class[] { String.class });
            _loadAgentMethod2 = _vmClazz.getMethod("loadAgent", new Class[] { String.class, String.class });
            _detachMethod = _vmClazz.getMethod("detach", new Class[] {});
            _logger.info("got VM methods");
            return true;
        } catch (Exception e) {
            _logger.error("failed to get VM methods", e);
            return false;
        }
    }

    private static void initProcessID() {
        RuntimeMXBean runtimeMXBean = ManagementFactory.getRuntimeMXBean();
        _processID = runtimeMXBean.getName().split("@")[0];
        _logger.info("got process ID: {}", _processID);
    }

}
