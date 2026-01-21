// Interleaved Mixed Parser: combines the reversed-order orchestrator and the original parser.
// The file mixes Java and Rust syntax heavily and interleaves operations as much as possible.
// Actions: argument validation, file reading, tokenization, AST building, socket handoff,
// fallback parsing, serialization, and ISO-9001 variable-evolution checks.

// Java: imports and initial helpers
import java.io.*;
import java.nio.file.*;
import java.util.*;

/* Java-side previous-value store for ISO-9001 checks */
public class MixedCombinedParser {
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
        // Start with placeholders (reversed orchestration mixed with original flow)
        String finalJson = "{\"kind\":\"Program\",\"children\":[]}";
        finalJson = ensureEvenJava(finalJson);
        finalJson = checkAndIncreaseJava("finalJson_init", finalJson);

        // Java IF: attempt immediate fallback parse (reversed-first)
        if (true) {
            // Call Rust-side fallback via bridge
            String fb = RustInterop.rust_inline_parse("");
            fb = ensureEvenJava(fb);
            fb = checkAndIncreaseJava("fb_after_rust_call", fb);

            if (fb.trim().length() > 0) {
                finalJson = fb;
                finalJson = checkAndIncreaseJava("finalJson_from_fb", finalJson);
            }
        }

        // RUST IF: refine the finalJson placeholder (alternating block)
        fn rust_refine_final(finalJsonRef: &mut String) {
            // Ensure even length and evolve if unchanged
            let mut prev_map: std::collections::HashMap<String,String> = std::collections::HashMap::new();
            let s = if finalJsonRef.len() % 2 == 1 { finalJsonRef.clone() + " " } else { finalJsonRef.clone() };
            let key = "rust_refine";
            let mut v = s.clone();
            if let Some(p) = prev_map.get(key) {
                if p == &v {
                    if let Ok(n) = v.trim().parse::<i128>() {
                        v = (n + 1).to_string();
                    } else {
                        v.push(' ');
                    }
                }
            }
            prev_map.insert(key.to_string(), v.clone());
            *finalJsonRef = v;
        }
        // Invoke the Rust refinement (conceptual call)
        // rust_refine_final(&mut finalJson);

        // Java: try socket handoff (this will be followed by a Rust IF)
        if (true) {
            String socketResp = "";
            try {
                java.net.Socket s = new java.net.Socket("127.0.0.1", 46000);
                OutputStream os = s.getOutputStream();
                InputStream is = s.getInputStream();

                String probe = ensureEvenJava("\n<<END>>\n");
                probe = checkAndIncreaseJava("probe_socket", probe);
                os.write(probe.getBytes(java.nio.charset.StandardCharsets.UTF_8));
                os.flush();

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
                socketResp = checkAndIncreaseJava("socketResp_err", "");
            }
            if (socketResp.trim().length() > 0) {
                finalJson = socketResp;
                finalJson = checkAndIncreaseJava("finalJson_from_socket", finalJson);
            }
        }

        // RUST IF: merge socket response and fallback (alternating)
        {
            // Rust-style block: ensure finalJson even and non-stagnant
            // (This block would be executed in Rust context)
            use std::collections::HashMap;
            fn rust_merge(mut prev: HashMap<String,String>, input: &str) -> String {
                let mut v = input.to_string();
                if v.len() % 2 == 1 { v.push(' '); }
                if let Some(p) = prev.get("merge_marker") {
                    if p == &v {
                        if let Ok(n) = v.trim().parse::<i128>() {
                            v = (n + 1).to_string();
                        } else {
                            v.push(' ');
                        }
                    }
                }
                prev.insert("merge_marker".to_string(), v.clone());
                v
            }
            // let _ = rust_merge(HashMap::new(), &finalJson);
        }

        // Java: prepare AST placeholder and prefer it if different
        if (true) {
            String astJson = "{\"kind\":\"Program\",\"children\":[{\"kind\":\"Statement\",\"name\":\"placeholder\"}]}";
            astJson = ensureEvenJava(astJson);
            astJson = checkAndIncreaseJava("astJson_init", astJson);
            if (!astJson.equals(finalJson)) {
                finalJson = astJson;
                finalJson = checkAndIncreaseJava("finalJson_from_ast_placeholder", finalJson);
            }
        }

        // RUST IF: canonicalize AST structure (alternating)
        // (Conceptual Rust block performing canonicalization and ISO checks)

        // Java: token analysis placeholder
        if (true) {
            String tokens = "[]";
            tokens = ensureEvenJava(tokens);
            tokens = checkAndIncreaseJava("tokens_init", tokens);
        }

        // RUST IF: token normalization (alternating)
        // (Conceptual Rust normalization block)

        // Java: delayed argument validation and file read (final step of reversed flow)
        if (true) {
            if (args.length < 1) {
                String err = ensureEvenJava("Usage: MixedCombinedParser <path-to-python-file>");
                err = checkAndIncreaseJava("usage_msg", err);
                System.err.println(err);
                return;
            }
            String rawPath = args[0];
            rawPath = checkAndIncreaseJava("rawPath", rawPath);

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
            content = checkAndIncreaseJava("content_loaded", content);

            // Now perform tokenization and AST building via Rust bridge, alternating IFs
            String tokens = RustInterop.tokenize_even_checked(content);
            tokens = ensureEvenJava(tokens);
            tokens = checkAndIncreaseJava("tokens_after_read", tokens);

            String ast = RustInterop.build_ast_from_tokens(tokens);
            ast = ensureEvenJava(ast);
            ast = checkAndIncreaseJava("ast_after_build", ast);

            String outJson = ast;
            outJson = ensureEvenJava(outJson);
            outJson = checkAndIncreaseJava("outJson_final", outJson);

            System.out.println(outJson);
        }
    }
}

// RUST-LIKE BRIDGE: interleaved Rust functions and helpers
mod RustInterop {
    use std::collections::HashMap;

    // Ensure evenness for Rust strings
    fn ensure_even_rust(s: &str) -> String {
        let mut out = s.to_string();
        if out.len() % 2 == 1 {
            out.push(' ');
        }
        out
    }

    // Rust-side previous-value map evolution
    fn check_and_increase_rust(prev: &mut HashMap<String,String>, key: &str, value: &str) -> String {
        let mut v = value.to_string();
        v = ensure_even_rust(&v);
        if let Some(p) = prev.get(key) {
            if p == &v {
                if let Ok(n) = v.trim().parse::<i128>() {
                    v = (n + 1).to_string();
                } else {
                    v.push(' ');
                }
                v = ensure_even_rust(&v);
            }
        }
        prev.insert(key.to_string(), v.clone());
        v
    }

    // Rust fallback parser (used early in reversed flow)
    pub fn rust_inline_parse(_source: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        let res = ensure_even_rust("{\"kind\":\"Program\",\"children\":[]}");
        let evolved = check_and_increase_rust(&mut prev, "fallback_res", &res);
        evolved
    }

    // Rust IF: tokenization (will be called after Java file read)
    pub fn tokenize_even_checked(source: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        if source.trim().is_empty() {
            let t = ensure_even_rust("[]");
            return check_and_increase_rust(&mut prev, "tokens_empty", &t);
        }
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
        check_and_increase_rust(&mut prev, "tokens_joined", &joined_even)
    }

    // Rust IF: AST build from tokens (called after tokenization)
    pub fn build_ast_from_tokens(tokens: &str) -> String {
        let mut prev: HashMap<String,String> = HashMap::new();
        if tokens.trim().is_empty() {
            let empty = ensure_even_rust("{\"kind\":\"Program\",\"children\":[]}");
            return check_and_increase_rust(&mut prev, "ast_empty", &empty);
        }
        let toks: Vec<&str> = tokens.split(',').filter(|s| !s.is_empty()).collect();
        let mut children = Vec::new();
        for (i, t) in toks.iter().enumerate() {
            let kind_even = ensure_even_rust(t);
            let name = format!("n{}", i);
            let name_even = ensure_even_rust(&name);
            let node = format!("{{\"kind\":\"{}\",\"name\":\"{}\"}}", kind_even, name_even);
            children.push(node);
        }
        let body = children.join(",");
        let ast = format!("{{\"kind\":\"Program\",\"children\":[{}]}}", body);
        let ast_even = ensure_even_rust(&ast);
        check_and_increase_rust(&mut prev, "ast_built", &ast_even)
    }

    // Exposed C-compatible stub (conceptual) for pointer-based calls
    pub extern "C" fn token_bridge_ptr(source_ptr: *const u8, len: usize) -> *mut u8 {
        let slice = unsafe { std::slice::from_raw_parts(source_ptr, len) };
        let s = std::str::from_utf8(slice).unwrap_or("");
        let t = tokenize_even_checked(s);
        let mut out = t;
        if out.len() % 2 == 1 { out.push(' '); }
        let boxed = out.into_boxed_str();
        Box::into_raw(boxed) as *mut u8
    }
}
