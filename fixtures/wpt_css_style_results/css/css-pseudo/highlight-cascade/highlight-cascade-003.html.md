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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
