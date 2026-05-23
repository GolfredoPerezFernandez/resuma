use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Bundle benchmark"</h1>
            <p class="lead">
                "Measured JavaScript for a resumable counter page. "
                "Static pages (like this docs landing) ship "
                <strong>"zero"</strong>" client JS."
            </p>

            <h2>"Methodology"</h2>
            <ol>
                <li>"Same UX: SSR heading + one interactive counter button."</li>
                <li>"Chrome DevTools → Network, disable cache, hard reload."</li>
                <li>"Compare transfer size with gzip/brotli enabled (production server)."</li>
                <li>"Report raw (uncompressed) and compressed bytes separately."</li>
            </ol>

            <h2>"Resuma (split runtime)"</h2>
            <table class="compare">
                <thead>
                    <tr>
                        <th>"Bundle"</th>
                        <th>"When loaded"</th>
                        <th>"Raw"</th>
                        <th>"Gzip"</th>
                        <th>"Brotli"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"loader.js"</code></td>
                        <td>"Interactive pages only"</td>
                        <td>"~1.8 KiB"</td>
                        <td>"~884 B"</td>
                        <td>"~730 B"</td>
                    </tr>
                    <tr>
                        <td><code>"core.js"</code></td>
                        <td>"First interaction or reactive bindings"</td>
                        <td>"~6.6 KiB"</td>
                        <td>"~2.6 KiB"</td>
                        <td>"~2.3 KiB"</td>
                    </tr>
                    <tr>
                        <td><strong>"Static docs page"</strong></td>
                        <td>"Never"</td>
                        <td class="yes">"0 B"</td>
                        <td class="yes">"0 B"</td>
                        <td class="yes">"0 B"</td>
                    </tr>
                </tbody>
            </table>

            <h2>"Reproduce locally"</h2>
            {code_block(r#"# Measure embedded bundles (raw + gzip + brotli)
cd runtime && npm run build && npm run size

# Live JSON from the dev server
curl -H "Accept-Encoding: gzip" http://127.0.0.1:3000/_resuma/benchmark.json

# Resuma counter (interactive — loader + core on first click)
cargo run -p example-counter"#)}

            <h2>"Takeaways"</h2>
            <ul>
                <li><strong>"Static-first:"</strong>" Resuma skips loader, payload, and runtime on pages with no interactivity."</li>
                <li><strong>"Small loader:"</strong>" loader.js stays under 1 KiB gzip on a typical counter page."</li>
                <li><strong>"Honest totals:"</strong>" Full interactivity still loads core.js — report loader + core, not just the loader."</li>
                <li><strong>"Production:"</strong>" Asset routes serve gzip/brotli based on Accept-Encoding."</li>
            </ul>
        </>
    }
}
