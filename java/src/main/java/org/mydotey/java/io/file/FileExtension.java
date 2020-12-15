package org.mydotey.java.io.file;

import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

import org.mydotey.java.ObjectExtension;
import org.mydotey.java.StringExtension;

/**
 * @author koqizhao
 *
 *         Dec 10, 2018
 */
public interface FileExtension {

	static String readFileContent(Path file) throws IOException {
		return readFileContent(file, StandardCharsets.UTF_8);
	}

	static String readFileContent(Path file, Charset charset) throws IOException {
		ObjectExtension.requireNonNull(file, "file");
		ObjectExtension.requireNonNull(charset, "charset");

		byte[] data = Files.readAllBytes(file);
		return new String(data, charset);
	}

	static void writeFileContent(Path file, String content) throws IOException {
		writeFileContent(file, content, StandardCharsets.UTF_8);
	}

	static void writeFileContent(Path file, String content, Charset charset) throws IOException {
		ObjectExtension.requireNonNull(file, "file");
		ObjectExtension.requireNonNull(charset, "charset");

		byte[] data = content == null ? new byte[] {} : content.getBytes(charset);
		Files.write(file, data, StandardOpenOption.CREATE);
	}

	static String concatPathParts(String... pathParts) {
		if (pathParts == null)
			return null;

		String url = null;
		for (String item : pathParts) {
			if (StringExtension.isBlank(item))
				continue;
			item = item.trim();

			if (url == null)
				url = item;
			else
				url += (url.endsWith("/") ? StringExtension.EMPTY : "/")
						+ StringExtension.nullToEmpty(StringExtension.trimStart(item, '/'));
		}

		return url;
	}

}
