/* odp interactive bits — pure SSG site, no framework runtime.
 *
 * Three jobs:
 *   1. Theme toggle: flip `<html data-theme>` between light/dark and
 *      persist the choice in localStorage["odp-theme"]. The initial
 *      theme is already set by the inline THEME_FLASH_SCRIPT in
 *      `<head>` before any CSS loads, so first paint is correct.
 *   2. Mobile nav drawer: hamburger button shows/hides the drawer,
 *      ESC and backdrop-click close it.
 *   3. Video facade: click-to-load YouTube embed (no third-party JS
 *      loaded until the user opts in).
 */

(function () {
    'use strict';

    // --- theme toggle ---------------------------------------------
    var THEME_KEY = 'odp-theme';
    function applyTheme(t) {
        document.documentElement.setAttribute('data-theme', t);
        try { localStorage.setItem(THEME_KEY, t); } catch (_) {}
    }
    document.querySelectorAll('[data-theme-toggle]').forEach(function (btn) {
        btn.addEventListener('click', function () {
            var current = document.documentElement.getAttribute('data-theme') === 'dark'
                ? 'dark' : 'light';
            applyTheme(current === 'dark' ? 'light' : 'dark');
        });
    });

    // --- mobile nav drawer ---------------------------------------
    var navToggle = document.querySelector('[data-mobile-nav-toggle]');
    var navDrawer = document.getElementById('primary-mobile-nav');
    var navBackdrop = document.querySelector('[data-mobile-nav-backdrop]');
    function setNav(open) {
        if (!navToggle || !navDrawer || !navBackdrop) return;
        navToggle.setAttribute('aria-expanded', String(open));
        navToggle.setAttribute('aria-label', open ? 'Close menu' : 'Open menu');
        if (open) {
            navDrawer.removeAttribute('hidden');
            navDrawer.style.display = 'flex';
            navBackdrop.removeAttribute('hidden');
        } else {
            navDrawer.setAttribute('hidden', '');
            navDrawer.style.display = '';
            navBackdrop.setAttribute('hidden', '');
        }
    }
    if (navToggle) {
        navToggle.addEventListener('click', function () {
            var open = navToggle.getAttribute('aria-expanded') === 'true';
            setNav(!open);
        });
    }
    if (navBackdrop) {
        navBackdrop.addEventListener('click', function () { setNav(false); });
    }
    document.addEventListener('keydown', function (e) {
        if (e.key === 'Escape' && navToggle &&
            navToggle.getAttribute('aria-expanded') === 'true') {
            setNav(false);
        }
    });
    // Close drawer when a link inside it is followed.
    if (navDrawer) {
        navDrawer.querySelectorAll('a').forEach(function (a) {
            a.addEventListener('click', function () { setNav(false); });
        });
    }

    // --- video facade --------------------------------------------
    document.querySelectorAll('[data-video-facade]').forEach(function (facade) {
        var btn = facade.querySelector('[data-video-play]');
        var tpl = facade.querySelector('template[data-video-iframe]');
        if (!btn || !tpl) return;
        btn.addEventListener('click', function () {
            var html = tpl.innerHTML;
            var host = document.createElement('div');
            host.className = 'w-full h-full';
            host.innerHTML = html;
            btn.replaceWith(host);
        });
    });
})();
