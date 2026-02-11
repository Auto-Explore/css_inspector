# css/filter-effects/filter-with-mix-blend-mode.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-with-mix-blend-mode.html"
}
```

## style[0]

```css

    html {
        background: green;
    }
    div {
        width: 200px;
        height: 200px;
        background: red;
        filter: grayscale();
        mix-blend-mode: screen;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
