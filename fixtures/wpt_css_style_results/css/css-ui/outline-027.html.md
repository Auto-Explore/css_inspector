# css/css-ui/outline-027.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-027.html"
}
```

## style[0]

```css

.outline-container {
  margin: 10px 0px;
  width: 10px;
  height: 10px;
  outline: auto;
  font: 10px/1 Ahem;
}

.outline-container > span > div {
  color: lime;
}

.inline-block-text > .outline-container > span > div {
  display: inline-block;
}

.inline-text > .outline-container > span > div {
  display: inline;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
