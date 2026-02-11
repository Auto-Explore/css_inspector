# css/css-variables/revert-in-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/revert-in-fallback.html"
}
```

## style[0]

```css

  body.revert {
    --x:FAIL;
    margin: -1px;
    display: grid;

    --x: var(--unknown, revert);
    margin: var(--unknown, revert);
    display: var(--unknown, revert);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
