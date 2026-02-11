# css/css-conditional/container-queries/style-query-registered-custom-invalid.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-registered-custom-invalid.tentative.html"
}
```

## style[0]

```css

  :root { container-name: --root; }
  @property --prop {
    syntax: "a";
    initial-value: a;
    inherits: true;
  }
  #target { background-color: green; }
  @container --root style(--prop: b) {
    #target { background-color: red; }
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
