# css/css-values/if-style-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-style-invalidation.html"
}
```

## style[0]

```css

  html {
    --x: 3;
  }
  #test {
    --prop: if(style(--x: 3): true_value; else: false_value;);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
