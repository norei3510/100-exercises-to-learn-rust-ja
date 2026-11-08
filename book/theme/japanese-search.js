/* global path_to_root */
(() => {
    "use strict";

    const input = document.getElementById("mdbook-searchbar");
    const results = document.getElementById("mdbook-searchresults");
    const header = document.getElementById("mdbook-searchresults-header");
    const outer = document.getElementById("mdbook-searchresults-outer");
    const form = document.getElementById("mdbook-searchbar-outer");
    if (!input || !results || !header || !outer || !form) return;

    // Keep mdBook's UI and English search. Its original documents also allow
    // Japanese substring searches without the English tokenizer/stemmer.
    const japanese = /[\u3040-\u30ff\u3400-\u9fff\uff66-\uff9f]/;
    let documents;
    let scheduled = false;
    input.placeholder = "日本語・Rustの語句を検索…";
    header.setAttribute("aria-live", "polite");

    function getDocuments() {
        if (documents) return documents;
        const config = window.search;
        const docs = config?.index?.documentStore?.docs;
        if (!docs || !config.doc_urls) return null;
        documents = Object.entries(docs).map(([id, doc]) => ({
            ...doc,
            url: config.doc_urls[id],
            titleLower: doc.title.toLowerCase(),
            textLower: `${doc.title} ${doc.breadcrumbs} ${doc.body}`.toLowerCase(),
        }));
        return documents;
    }

    function snippet(body, terms) {
        const text = body.replace(/\s+/g, " ").trim();
        const lower = text.toLowerCase();
        const positions = terms.map(term => lower.indexOf(term)).filter(pos => pos >= 0);
        const start = positions.length ? Math.max(0, Math.min(...positions) - 45) : 0;
        const end = Math.min(text.length, start + 170);
        return (start ? "…" : "") + text.slice(start, end) + (end < text.length ? "…" : "");
    }

    function render() {
        scheduled = false;
        const query = input.value.trim();
        if (!japanese.test(query)) return;
        const docs = getDocuments();
        if (!docs) return; // mdBook loads its index when search first opens.
        const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
        const matches = docs.filter(doc => doc.url && terms.every(term => doc.textLower.includes(term)));
        const score = doc => terms.reduce((total, term) => total +
            (doc.titleLower === term ? 100 : doc.titleLower.includes(term) ? 30 : 0), 0);
        matches.sort((a, b) => score(b) - score(a));
        const limit = window.search.results_options?.limit_results || 30;

        header.textContent = `「${query}」の検索結果：${matches.length}件` +
            (matches.length > limit ? `（先頭${limit}件を表示）` : "");
        results.replaceChildren();
        matches.slice(0, limit).forEach((doc, index) => {
            const item = document.createElement("li");
            const link = document.createElement("a");
            const teaser = document.createElement("span");
            const url = new URL(path_to_root + doc.url, window.location.href);
            url.searchParams.set("highlight", query);
            link.href = url.href;
            link.textContent = doc.breadcrumbs || doc.title;
            teaser.className = "teaser";
            teaser.id = `japanese-search-teaser-${index}`;
            teaser.textContent = snippet(doc.body, terms);
            teaser.setAttribute("aria-label", "検索結果の抜粋");
            link.setAttribute("aria-details", teaser.id);
            item.append(link, teaser);
            results.append(item);
        });
        outer.classList.remove("hidden");
        form.classList.remove("searching");
    }

    function schedule() {
        if (scheduled) return;
        scheduled = true;
        window.requestAnimationFrame(render);
    }

    input.addEventListener("keyup", schedule);
    input.addEventListener("input", event => {
        // Let mdBook update URL/history and handle clearing on mobile as well.
        if (!event.isComposing) input.dispatchEvent(new Event("keyup"));
    });
    input.addEventListener("compositionend", () => input.dispatchEvent(new Event("keyup")));
    window.addEventListener("popstate", schedule);

    // The index can arrive before or after this additional script. Run after
    // mdBook's own onload handler has initialized and rendered its results.
    const observer = new MutationObserver(watchIndex);
    function watchIndex() {
        const script = document.getElementById("mdbook-search-index");
        if (!script) return;
        script.addEventListener("load", schedule, { once: true });
        observer.disconnect();
        schedule();
    }
    observer.observe(document.head, { childList: true });
    watchIndex();
    schedule();
})();
