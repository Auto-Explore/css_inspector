# css/css-conditional/at-supports-namespace-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-namespace-002.html"
}
```

## style[0]

```css

		@namespace x "http://www.w3.org/1999/xlink";
		@supports (background: green) {}
		@namespace y "http://www.w3.org/1999/xlink";

		div {
			height: 25px;
			width: 100px;
		}

		.test1 { background: red; }
		@supports selector(x|y) {
			.test1 { background: green; }
		}

		.test2 { background: green; }
		@supports selector(y|x) {
			.test2 { background: red; }
		}

		.test3 { background: red; }
		@supports not selector(y|x) {
			.test3 { background: green; }
		}

		/* technically redundant with test 001, but need to add to 100px anyway */
		.test4, x|y { background: green; }
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
