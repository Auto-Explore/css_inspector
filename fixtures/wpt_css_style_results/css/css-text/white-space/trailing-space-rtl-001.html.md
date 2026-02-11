# css/css-text/white-space/trailing-space-rtl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/trailing-space-rtl-001.html"
}
```

## style[0]

```css

body {
  direction: rtl;
  white-space: pre-wrap;
}
.bg {
  background: orange;
}
.override {
  unicode-bidi: bidi-override;
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
