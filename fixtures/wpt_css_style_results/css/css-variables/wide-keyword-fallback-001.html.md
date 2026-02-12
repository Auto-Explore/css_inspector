# css/css-variables/wide-keyword-fallback-001.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/wide-keyword-fallback-001.html"
}
```

## style[0]

```css

  /* Should see a 10px border of the initial color */
  #outer {
    color: transparent;
    border: 10px solid;
  }

  #inner {
    color: var(--unknown, initial);
    border-width: 10px;
    border-style: var(--unknown, inherit);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
