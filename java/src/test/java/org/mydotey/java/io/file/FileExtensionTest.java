package org.mydotey.java.io.file;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author koqizhao
 *
 * Dec 12, 2018
 */
public class FileExtensionTest {

    @Test
    public void readFileContent() throws URISyntaxException, IOException {
        Path path = Paths.get(FileExtensionTest.class.getResource("test.txt").toURI());
        String expected = "1-1\n2-2";
        String actual = FileExtension.readFileContent(path);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void writeFileContent() throws URISyntaxException, IOException {
        Path path = Paths.get("temp.txt");
        String expected = "1-1\n2-2\n3-3";
        FileExtension.writeFileContent(path, expected);
        String actual = FileExtension.readFileContent(path);
        Assert.assertEquals(expected, actual);
        Files.delete(path);
    }

}
