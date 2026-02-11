# css/css-conditional/at-media-003.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-003.html"
}
```

## style[0]

```css

		@namespace x "http://www.w3.org/";
		@media { /* invalidates later rules even if empty */ }
		@import "support/fail.css";
		@namespace y "http://www.w3.org/";

		.test1, x|div { background: green; }
		.test1, y|div { background: red; }

		div {
			background: red;
			height: 50px;
			width: 100px;
		}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

		@media all {
			/* @supports isn't invalidated, only misordered stuff after it is */
			.test2 { background: green; }
		}
		@import "support/fail.css";
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
