# css/css-variables/variable-recalc-with-initial.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-recalc-with-initial.html"
}
```

## style[0]

```css

    .a {
      --color: red;
    }
    .b {
      --color: initial;
    }
    .c {
      color: var(--color, green);
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
