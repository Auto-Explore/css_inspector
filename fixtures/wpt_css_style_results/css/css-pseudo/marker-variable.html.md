# css/css-pseudo/marker-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-variable.html"
}
```

## style[0]

```css

    .firstTest::marker {
      --alpha: 0.75;
      color: rgb(255 119 0 / var(--alpha));
    }

    .secondTest::marker {
      --color: rgb(255 119 0);
      color: var(--color);
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
