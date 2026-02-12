# css/css-values/if-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-invalidation.html"
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
