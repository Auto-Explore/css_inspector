# css/css-text/line-breaking/line-breaking-018.html

```json
{
  "format_version": 3,
  "file": "css/css-text/line-breaking/line-breaking-018.html"
}
```

## style[0]

```css

.outer { float: left; padding: 0 1em; margin: 1em; border: 1px solid silver; }

.outer div { font-family: monospace; line-height: 2; margin: 1em 0; border: 1px dotted silver; }

.test20 div { width: 20ch; }
.test27 div { width: 27ch; }
.test30 div { width: 30ch; }

b { color: blue; border-left: 2px solid red; border-right: 2px solid green; }

.test::before,
.test::after,
.before,
.after { position: absolute; }

.test::before { content: ""; }
.test::after { content: ""; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
