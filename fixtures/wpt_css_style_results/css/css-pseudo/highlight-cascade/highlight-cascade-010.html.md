# css/css-pseudo/highlight-cascade/highlight-cascade-010.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-010.html"
}
```

## style[0]

```css

    #originating::spelling-error {
        color: white;
        --x: initial;
        background-color: var(--x, red);
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
