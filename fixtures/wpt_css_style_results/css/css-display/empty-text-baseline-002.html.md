# css/css-display/empty-text-baseline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-display/empty-text-baseline-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.ahem {
  font-family: Ahem;
}

.with-pseudo::before, .with-pseudo::after {
  content: "";
  width: 4px;
  height: 4px;
  border: 2px solid;
  background: orange;
  display: inline-block;
}

.font-size::before, .font-size::after {
  font-size: 100px;
}

.line-height::before, .line-height::after {
  line-height: 10;
}

.empty-concat::before, .empty-concat::after {
  content: "" "" "" "" "";
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
