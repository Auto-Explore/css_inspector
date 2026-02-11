# css/css-conditional/at-media-content-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-content-001.html"
}
```

## style[0]

```css

		div {
			background: red;
			height: 20px;
			width: 100px;
		}

		@media all {
			@import "support/fail.css";
			.test1 { background: green; }
		}
		@media all {
			@namespace foo "http://www.w3.org/";
			.test2 { background: green; }
			.test2:not(foo|div) { background: red; }
		}
		@media all {
			@charset "utf-8";
			.test3 { background: green; }
		}
		@media all {
			@unknown;
			.test4 { background: green; }
		}
		@media all {
			.test5 { background: green; }
			@unknown {
				.test5 { background: red; }
			}
		}
	
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
