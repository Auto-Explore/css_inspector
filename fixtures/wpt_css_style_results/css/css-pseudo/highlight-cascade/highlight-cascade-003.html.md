# css/css-pseudo/highlight-cascade/highlight-cascade-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-003.html"
}
```

## style[0]

```css

  @property --bg {
    syntax: "<color>";
    inherits: false;
    initial-value: green;
  }
  :root::selection {
    background-color: var(--bg, red);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
