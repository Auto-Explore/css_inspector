# css/css-conditional/container-queries/style-query-registered-custom-rem-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-registered-custom-rem-change.html"
}
```

## style[0]

```css

  @property --length {
    syntax: "<length>";
    initial-value: 0px;
    inherits: false;
  }

  :root, body { font-size: 16px; }
  #container { --length: 100px; }

  #target { color: red; }
  @container style(--length: calc(1rem * 10)) {
    #target { color: green; }
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
