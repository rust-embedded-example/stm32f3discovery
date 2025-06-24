// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item "><a href="前言/前言.html"><strong aria-hidden="true">1.</strong> 前言</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="前言/Rust语言介绍.html"><strong aria-hidden="true">1.1.</strong> Rust语言介绍</a></li><li class="chapter-item "><a href="前言/Rust嵌入式.html"><strong aria-hidden="true">1.2.</strong> 嵌入式系统</a></li><li class="chapter-item "><a href="前言/Rust与AI编码助手.html"><strong aria-hidden="true">1.3.</strong> Rust与AI编码助手</a></li></ol></li><li class="chapter-item "><a href="环境搭建/环境搭建.html"><strong aria-hidden="true">2.</strong> 环境搭建</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="环境搭建/调试工具.html"><strong aria-hidden="true">2.1.</strong> 调试工具</a></li></ol></li><li class="chapter-item "><a href="PAC/PAC.html"><strong aria-hidden="true">3.</strong> PAC工程</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="PAC/创建基础工程.html"><strong aria-hidden="true">3.1.</strong> 创建基础工程</a></li><li class="chapter-item "><a href="PAC/创建PAC库.html"><strong aria-hidden="true">3.2.</strong> 创建PAC crate</a></li><li class="chapter-item "><a href="PAC/完善工程.html"><strong aria-hidden="true">3.3.</strong> 完善工程</a></li><li class="chapter-item "><a href="PAC/使用gdb调试.html"><strong aria-hidden="true">3.4.</strong> 使用gdb调试</a></li><li class="chapter-item "><a href="PAC/自定义链接器、链接脚本、内存.html"><strong aria-hidden="true">3.5.</strong> 自定义链接器、链接脚本、内存</a></li></ol></li><li class="chapter-item "><a href="HAL/HAL.html"><strong aria-hidden="true">4.</strong> HAL工程</a></li><li class="chapter-item "><a href="BSP/BSP.html"><strong aria-hidden="true">5.</strong> BSP工程</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
