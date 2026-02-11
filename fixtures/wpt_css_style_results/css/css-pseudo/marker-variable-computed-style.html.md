# css/css-pseudo/marker-variable-computed-style.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-variable-computed-style.html"
}
```

## style[0]

```css

        .firstTest::marker {
            --alpha: 0.75;
            color: rgba(0 128 0 / var(--alpha));
        }

        .secondTest::marker {
            --color: rgb(0 128 0);
            color: var(--color);
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
