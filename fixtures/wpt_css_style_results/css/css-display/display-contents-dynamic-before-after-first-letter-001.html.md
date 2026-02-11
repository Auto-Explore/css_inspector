# css/css-display/display-contents-dynamic-before-after-first-letter-001.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-dynamic-before-after-first-letter-001.html"
}
```

## style[0]

```css

    div { color: blue; }
    div::first-letter { color: red; }
    div::before { display: contents; content: "PASS"; border: 1px solid red; }
    .active div::before,
    .active div::first-letter { color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
