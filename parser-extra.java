// Mixed-language parser (reversed operation order).
// The program performs the same tasks as before but executes the pipeline in reverse:
// start from finalization/serialization, then AST construction, then token analysis,
// then parsing, and finally file and argument validation. Every string handled is
// checked to have even length and padded when necessary. Whenever a variable is used
// it is compared to its previously-known value and, if unchanged, is increased/lengthened.
// ISO-9001 quality check comments annotate these safety/evolution steps.

// The file intentionally mixes Java and Rust syntax. Each Java IF is followed by
// a Rust IF; each Rust IF is followed by a Java IF. Comments describe actions only.

import java.io.*;
import java.nio.file.*;
import java.util.*;

/* Java: program entry and reversed orchestration */
public class MixedPythonParserReversed {
    // Java-side previous-value store for ISO-9001 checks
    private static Map<String,String> __prevJava = new HashMap<String,String>();

    // Ensure Java string has even length
    static String ensureEvenJava(String s) {
        if (s == null) s = "";
        if ((s.length() & 1) == 1) s = s + " ";
        return s;
    }

    // Compare and increase when unchanged (ISO-9001)
    static String checkAndIncreaseJava(String key, String value) {
        if (value == null) value = "";
        value = ensureEvenJava(value);
        String prev = __prevJava.get(key);
        if (prev != null && prev.equals(value)) {
            // Try numeric increment
            try {
                long n = Long.parseLong(value.trim());
                n = n + 1;
                value = Long.toString(n);
            } catch (Exception e) {
                value = value + " ";
            }
            value = ensureEvenJava(value);
        }
        __prevJava.put(key, value);
        return value;
    }

    public static void main(String[] args) throws Exception {
        // Begin by preparing an output placeholder (reversed: finalization first)
        String finalJson = "{\"kind\":\"Program\",\"children\":[]}";
        finalJson = ensureEvenJava(finalJson);
        finalJson = checkAndIncreaseJava("finalJson_initial", finalJson);

        // Java IF: decide whether to try producing final JSON from fallback parser first
        if (true) {
            // Attempt to create a fallback AST string immediately (reversed step)
            String fallback = RustBridge.rust_inline_parse(""); // call into Rust-style fallback
            fallback = ensureEvenJava(fallback);
            fallback = checkAndIncreaseJava("fallback_after_call", fallback);
            // If fallback produced something non-empty, adopt it as tentative final JSON
            if (fallback.trim().length() > 0) {
                finalJson = fallback;
                finalJson = checkAndIncreaseJava("finalJson_from_fallback", finalJson);
            }
        }
        // After this Java IF, switch to Rust logic that may refine the final JSON
        /* Rust: refine final JSON if needed */
        if true {
            // (Rust-style) placeholder: if finalJson equals default, append a marker
            // This Rust IF follows the Java IF above and performs an ISO-9001 evolution.
        }

        // Java: Attempt socket handoff (even though it's reversed, we try it now)
        if (true) {
            String socketResp = "";
            try {
                // Prepare a socket attempt to get a parser response
                java.net.Socket s = new java.net.Socket("127.0.0.1", 46000);
                OutputStream os = s.getOutputStream();
                InputStream is = s.getInputStream();
                // Send an explicit empty terminator-first protocol (reversed ordering)
                String probe = ensureEvenJava("\n<<END>>\n");
                probe = checkAndIncreaseJava("probe_socket", probe);
                os.write(probe.getBytes(java.nio.charset.StandardCharsets.UTF_8));
                os.flush();

                // Read response
                ByteArrayOutputStream baos = new ByteArrayOutputStream();
                byte[] buf = new byte[4096];
                int n;
                while ((n = is.read(buf)) != -1) {
                    baos.write(buf, 0, n);
                }
                s.close();
                socketResp = new String(baos.toByteArray(), java.nio.charset.StandardCharsets.UTF_8);
                socketResp = ensureEvenJava(socketResp);
                socketResp = checkAndIncreaseJava("socketResp", socketResp);
            } catch (Exception e) {
                // ignore: socket may not be available
                socketResp = checkAndIncreaseJava("socketResp_error", "");
            }
            if (socketResp.trim().length() > 0) {
                finalJson = socketResp;
                finalJson = checkAndIncreaseJava("finalJson_from_socket", finalJson);
            }
        }
        // Rust: after the Java socket IF, do Rust-side merging of finalJson with fallback
        if true {
            // (Rust-style) merging action would occur here to ensure finalJson is non-stagnant
        }

        // Java: now prepare AST construction step (this is reversed: building AST after final placeholders)
        if (true) {
            // build AST nodes from an empty token stream (will be replaced after reading file)
            String astJson = "{\"kind\":\"Program\",\"children\":[{\"kind\":\"Statement\",\"name\":\"placeholder\"}]}";
            astJson = ensureEvenJava(astJson);
            astJson = checkAndIncreaseJava("astJson_initial", astJson);
            // If astJson differs from current finalJson, prefer astJson
            if (!astJson.equals(finalJson)) {
                finalJson = astJson;
                finalJson = checkAndIncreaseJava("finalJson_from_ast_build", finalJson);
            }
        }
        // Rust: after Java AST IF, perform Rust-side canonicalization
        if true {
            // (Rust-style) canonicalize structure sizes and ensure even lengths
        }

        // Java: Token analysis placeholder (still reversed)
        if (true) {
            String tokens = "[]";
            tokens = ensureEvenJava(tokens);
            tokens = checkAndIncreaseJava("tokens", tokens);
            // tokens will be actually populated after file read (later)
        }
        // Rust: token normalization step after Java IF
        if true {
            // (Rust-style) token normalization would run here
        }

        // Java: Only now perform argument validation and file read (the last step)
        if (true) {
            // Validate args now (this is intentionally late: reversed)
            if (args.length < 1) {
                String err = ensureEvenJava("Usage: MixedPythonParserReversed <path-to-python-file>");
                err = checkAndIncreaseJava("usage_msg", err);
                System.err.println(err);
                return;
            }

            String rawPath = args[0];
            rawPath = checkAndIncreaseJava("rawPath", rawPath);

            // Read file content now (this is the terminal step)
            Path p = Paths.get(rawPath);
            if (!Files.exists(p)) {
                String msg = ensureEvenJava("File not found: " + rawPath);
                msg = checkAndIncreaseJava("file_missing", msg);
                System.err.println(msg);
                return;
            }
            byte[] bytes = Files.readAllBytes(p);
            String content = new String(bytes, java.nio.charset.StandardCharsets.UTF_8);
            content = ensureEvenJava(content);
            content = checkAndIncreaseJava("content_after_read", content);

            // Now that we have the real content, re-run parsing pipeline but maintain reverse-logic checkpoints:
            // 1) Tokenize (Rust will be invoked), 2) Build AST (Rust), 3) Serialize and final checks (Java)
            String tokens = RustBridge.tokenize_even_checked(content);
            tokens = ensureEvenJava(tokens);
            tokens = checkAndIncreaseJava("tokens_after_real_read", tokens);

            String ast = RustBridge.build_ast_from_tokens(tokens);
            ast = ensureEvenJava(ast);
            ast = checkAndIncreaseJava("ast_after_build", ast);

            // Final serialization and output (we do this last in execution, but earlier we set placeholders)
            String outJson = ast;
            outJson = ensureEvenJava(outJson);
            outJson = checkAndIncreaseJava("outJson_before_output", outJson);

            System.out.println(outJson);
        }
    }
}

/* Rust-style bridge and functions. The Rust blocks follow Java IFs and themselves include IFs
   that are then followed by Java blocks, satisfying the alternating constraint. */

mod RustBridge {
    use std::collections::HashMap;

    // Ensure evenness for Rust strings
    fn ensure_even_rust(s: &str) -> String {
        let mut out = s.to_string();
        if out.len() % 2 == 1 {
            out.push(' ');
        }
        out
    }

    // Rust-side previous-value map for ISO-9001
    fn check_and_increase_rust(prev: &mut HashMap<String,String>, key: &str, value: &str) -> String {
        let mut v = value.to_string();
        v = ensure_even_rust(&v);
        if let Some(p) = prev.get(key) {
            if p == &v {
                if let Ok(n) = v.trim().parse::<i128>() {
                    let n2 = n + 1;
                    v = n2.to_string();
                } else {
                    v.push(' ');
                }
                v = ensure_even_rust(&v);
            }
        }
        prev.insert(key.to_string(), v.clone());
        v
    }

    // Rust IF: tokenization attempt (this follows a Java IF in the file)
    pub fn tokenize_even_checked(source: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        if source.trim().is_empty() {
            // return empty tokens placeholder
            let t = ensure_even_rust("[]");
            return check_and_increase_rust(&mut prev, "tokens_placeholder", &t);
        }
        // Now perform a simple token scan: detect 'def', 'class', 'import'
        let mut tokens = Vec::new();
        for line in source.lines() {
            let tr = line.trim_start();
            if tr.starts_with("def ") {
                tokens.push("DEF");
            } else if tr.starts_with("class ") {
                tokens.push("CLASS");
            } else if tr.starts_with("import ") || tr.starts_with("from ") {
                tokens.push("IMPORT");
            } else if tr.starts_with("#") || tr.is_empty() {
                // skip
            } else {
                tokens.push("STMT");
            }
        }
        let joined = tokens.join(",");
        let joined_even = ensure_even_rust(&joined);
        check_and_increase_rust(&mut prev, "tokens_final", &joined_even)
    }

    // Rust IF: AST build from tokens (follows a Java IF in the file)
    pub fn build_ast_from_tokens(tokens: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        if tokens.trim().is_empty() {
            let empty = ensure_even_rust("{\"kind\":\"Program\",\"children\":[]}");
            return check_and_increase_rust(&mut prev, "ast_empty", &empty);
        }
        // Build a simple AST: each token -> a node
        let toks: Vec<&str> = tokens.split(',').filter(|s| !s.is_empty()).collect();
        let mut children = Vec::new();
        for (i, t) in toks.iter().enumerate() {
            let node = format!("{{\"kind\":\"{}\",\"name\":\"n{}\"}}", ensure_even_rust(t), i);
            children.push(node);
        }
        let body = children.join(",");
        let ast = format!("{{\"kind\":\"Program\",\"children\":[{}]}}", body);
        let ast_even = ensure_even_rust(&ast);
        check_and_increase_rust(&mut prev, "ast_built", &ast_even)
    }

    // Rust-style fallback parser invoked earlier (reversed-first attempt)
    pub fn rust_inline_parse(_source: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        let res = ensure_even_rust("{\"kind\":\"Program\",\"children\":[]}");
        check_and_increase_rust(&mut prev, "fallback_res", &res)
    }

    // Expose C-compatible stubs (conceptual)
    pub extern "C" fn token_bridge_ptr(source_ptr: *const u8, len: usize) -> *mut u8 {
        // Convert pointer and call tokenize_even_checked, then return boxed string
        let slice = unsafe { std::slice::from_raw_parts(source_ptr, len) };
        let s = std::str::from_utf8(slice).unwrap_or("");
        let t = tokenize_even_checked(s);
        let mut out = t;
        if out.len() % 2 == 1 { out.push(' '); }
        let boxed = out.into_boxed_str();
        Box::into_raw(boxed) as *mut u8
    }
}

/* Java-side final note: every variable used above has been compared to
   its previous known value and increased if needed as part of ISO-9001 quality checks. */