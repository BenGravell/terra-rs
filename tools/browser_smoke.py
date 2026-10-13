"""Smoke-test the canvas app, persistence, and narrow layout at its real subpath."""
import argparse
from functools import partial
import gzip
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import shutil
from threading import Thread

from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parent.parent


class Handler(SimpleHTTPRequestHandler):
    def do_GET(self):
        if not self.path.startswith("/terra-rs/"):
            self.send_error(404)
            return
        self.path = self.path[len("/terra-rs"):]
        super().do_GET()

    def log_message(self, *_):
        pass


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--browser", choices=["chromium", "firefox", "webkit"], default="chromium")
    parser.add_argument("--url", help="Test a deployed site instead of serving dist locally")
    args = parser.parse_args()
    artifacts = ROOT / "artifacts"
    artifacts.mkdir(exist_ok=True)
    server = ThreadingHTTPServer(("127.0.0.1", 0), partial(Handler, directory=str(ROOT / "dist")))
    Thread(target=server.serve_forever, daemon=True).start()
    url = args.url or f"http://127.0.0.1:{server.server_port}/terra-rs/"
    with sync_playwright() as p:
        options = {"headless": True}
        if args.browser == "chromium":
            if executable := shutil.which("google-chrome"):
                options["executable_path"] = executable
            options["args"] = ["--no-sandbox", "--enable-unsafe-swiftshader", "--use-gl=angle", "--use-angle=swiftshader"]
        browser = getattr(p, args.browser).launch(**options)
        page = browser.new_page(viewport={"width": 1440, "height": 1000})
        errors = []
        page.on("pageerror", lambda error: errors.append(str(error)))

        def ready():
            page.wait_for_selector("html[data-terra-ready]", timeout=120_000)
            assert page.locator("#app").is_visible()
            page.wait_for_function("document.getElementById('loading').hidden")

        def settings():
            return page.evaluate("JSON.parse(localStorage.getItem('terra.settings.v1')).settings")

        def wait_settings(expression):
            page.wait_for_function(f"{expression}")

        try:
            page.goto(url)
            ready()
            initial = settings()
            startup = page.evaluate("performance.getEntriesByName('terra-ready')[0].startTime")
            page.screenshot(path=str(artifacts / f"{args.browser}-desktop.png"))
            # Canvas coordinates for Power distance and Reset at the fixed desktop viewport.
            page.mouse.move(76, 225)
            page.mouse.down()
            page.mouse.move(122, 225, steps=8)
            page.mouse.up()
            wait_settings("JSON.parse(localStorage.getItem('terra.settings.v1')).settings.culture[0] > 50")
            changed = settings()
            page.reload()
            ready()
            assert settings() == changed
            page.mouse.click(210, 75, delay=100)
            wait_settings("JSON.parse(localStorage.getItem('terra.settings.v1')).settings.culture[0] === 50")
            assert settings() == initial
            # Culture Fit slider drives the other overall weight through the actual UI.
            page.mouse.move(112, 717)
            # Let egui hit-test the new pointer position, then hold until it handles the press.
            page.evaluate("new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))")
            page.mouse.down()
            wait_settings("JSON.parse(localStorage.getItem('terra.settings.v1')).settings.overall_weights[0] > .5")
            page.mouse.up()
            weights = settings()["overall_weights"]
            assert abs(sum(weights) - 1) < 1e-9 and all(0 <= w <= 1 for w in weights)
            page.reload()
            ready()
            assert all(abs(a - b) < 1e-9 for a, b in zip(settings()["overall_weights"], weights, strict=True))
            page.mouse.click(210, 75, delay=100)
            wait_settings("JSON.parse(localStorage.getItem('terra.settings.v1')).settings.overall_weights[0] === .5")
            page.set_viewport_size({"width": 390, "height": 844})
            page.wait_for_timeout(400)
            page.screenshot(path=str(artifacts / f"{args.browser}-narrow.png"))
            page.mouse.click(155, 24, delay=100)  # Settings tab.
            page.wait_for_timeout(200)
            page.mouse.click(122, 225, delay=100)
            wait_settings("JSON.parse(localStorage.getItem('terra.settings.v1')).settings.culture[0] > 50")
            page.screenshot(path=str(artifacts / f"{args.browser}-narrow-settings.png"))
            # Malformed storage recovers and persists defaults.
            page.evaluate("localStorage.setItem('terra.settings.v1', 'broken')")
            page.reload()
            ready()
            assert settings() == initial
            # Storage-denied browsers still start and render the canvas.
            page.add_init_script("Object.defineProperty(window, 'localStorage', {get() { throw new DOMException('Blocked'); }});")
            page.reload()
            ready()
            assert not errors, errors
            result = {"browser": args.browser, "version": browser.version, "startup_ms": startup, "viewport": "1440x1000 and 390x844", "checks": "passed", "url": url}
            if not args.url:
                wasm = next((ROOT / "dist").glob("*_bg.wasm")).read_bytes()
                result.update(wasm_bytes=len(wasm), wasm_gzip_bytes=len(gzip.compress(wasm)))
            (artifacts / f"{args.browser}-results.json").write_text(json.dumps(result, indent=2) + "\n")
            print(json.dumps(result))
        except Exception:
            page.screenshot(path=str(artifacts / f"{args.browser}-failure.png"))
            print("Browser errors:", errors)
            raise
        finally:
            browser.close()
            server.shutdown()


if __name__ == "__main__":
    main()
