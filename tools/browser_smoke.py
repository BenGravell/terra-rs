"""Smoke-test the canvas app, persistence, and narrow layout at its real subpath."""
import argparse
from functools import partial
import gzip
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
from threading import Thread

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


def layout_plan():
    # Generate fresh inputs; never reuse coordinates from an earlier build.
    with tempfile.TemporaryDirectory(prefix="terra-smoke-") as directory:
        path = Path(directory) / "plan.json"
        subprocess.run(
            ["cargo", "test", "--locked", "--lib", "ui::browser_smoke_test::browser_smoke_contract", "--", "--exact"],
            cwd=ROOT, env={**os.environ, "TERRA_SMOKE_PLAN": str(path)}, check=True,
        )
        plan = json.loads(path.read_text())
    assert plan, "Native smoke test produced no steps"
    for step in plan:
        assert step["action"] in {"click", "reload", "resize", "screenshot"}, step
    return plan


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--browser", choices=["chromium", "firefox", "webkit"], default="chromium")
    parser.add_argument("--url", help="Test a deployed site instead of serving dist locally")
    parser.add_argument("--check-layout", action="store_true", help="Only run the native layout/input pre-check")
    args = parser.parse_args()
    plan = layout_plan()
    if args.check_layout:
        return
    from playwright.sync_api import sync_playwright

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
        viewport = {key: plan[0][key] for key in ("width", "height")}
        page = browser.new_page(viewport=viewport)
        errors = []
        page.on("pageerror", lambda error: errors.append(str(error)))

        def ready():
            page.wait_for_selector("html[data-terra-ready]", timeout=120_000)
            assert page.locator("#app").is_visible()
            page.wait_for_function("document.getElementById('loading').hidden")

        def settings():
            return page.evaluate("JSON.parse(localStorage.getItem('terra.settings'))")

        def click(step):
            page.mouse.move(*step["point"])
            # Give egui a frame to hit-test before pressing.
            page.evaluate("new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))")
            page.mouse.down()
            try:
                if not step["hold"]:
                    page.wait_for_timeout(100)
                    page.mouse.up()
                page.wait_for_function(
                    "expected => JSON.stringify(JSON.parse(localStorage.getItem('terra.settings'))) === JSON.stringify(expected)",
                    arg=step["settings"], timeout=5_000,
                )
            except Exception as error:
                raise AssertionError(f"{step['label']} at {step['point']}: expected {step['settings']}, got {settings()}") from error
            finally:
                if step["hold"]:
                    page.mouse.up()
            # Collapsing sections animate before subsequent targets settle.
            page.wait_for_timeout(300)

        try:
            page.goto(url)
            ready()
            initial = settings()
            startup = page.evaluate("performance.getEntriesByName('terra-ready')[0].startTime")
            for step in plan:
                match step["action"]:
                    case "click":
                        click(step)
                    case "reload":
                        page.reload()
                        ready()
                        assert settings() == step["settings"], f"Reload: expected {step['settings']}, got {settings()}"
                    case "resize":
                        page.set_viewport_size({"width": step["width"], "height": step["height"]})
                        page.wait_for_timeout(400)
                    case "screenshot":
                        page.screenshot(path=str(artifacts / f"{args.browser}-{step['name']}.png"))
            # Malformed storage recovers and persists defaults.
            page.evaluate("localStorage.setItem('terra.settings', 'broken')")
            page.reload()
            ready()
            assert settings() == initial
            # Storage-denied browsers still start and render the canvas.
            page.add_init_script("Object.defineProperty(window, 'localStorage', {get() { throw new DOMException('Blocked'); }});")
            page.reload()
            ready()
            assert not errors, errors
            viewports = " and ".join(f"{step['width']}x{step['height']}" for step in plan if step["action"] == "resize")
            result = {"browser": args.browser, "version": browser.version, "startup_ms": startup, "viewport": viewports, "checks": "passed", "url": url}
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
