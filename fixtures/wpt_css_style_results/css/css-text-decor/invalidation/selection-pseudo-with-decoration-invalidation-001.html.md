# css/css-text-decor/invalidation/selection-pseudo-with-decoration-invalidation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/invalidation/selection-pseudo-with-decoration-invalidation-001.html"
}
```

## style[0]

```css

    div {
        display: block;
        margin-top: 30px;
        margin-bottom: 30px;
        will-change: transform;
    }
    ::selection {
        background: yellow;
        color: currentColor;
        text-decoration-line: underline overline;
        text-decoration-style: wavy;
        text-decoration-color: black;
        text-decoration-thickness: 5px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
