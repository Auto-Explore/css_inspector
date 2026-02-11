# css/css-variables/wide-keyword-fallback-002.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/wide-keyword-fallback-002.html"
}
```

## style[0]

```css

  .outer1 {
    --color: green;
  }
  .outer2 {
    --color: var(--foo, unset);
  }
  .inner {
    color: var(--color);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
