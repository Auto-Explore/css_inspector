# css/css-variables/whitespace-in-fallback-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/whitespace-in-fallback-crash.html"
}
```

## style[0]

```css

  #t1 {--a:var(--b,var(--c,) );}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
