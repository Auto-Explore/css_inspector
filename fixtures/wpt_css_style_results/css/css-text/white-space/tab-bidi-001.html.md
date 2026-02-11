# css/css-text/white-space/tab-bidi-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/tab-bidi-001.html"
}
```

## style[0]

```css

body > div {
  border: solid blue;
  margin: 1em;
}
div > div {
  white-space: pre;
  font: 20px/1 monospace;
  text-align: left;
}
div > span { /* measure bounds */
  border: orange;
  border-style: none solid;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
