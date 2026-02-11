# css/css-text-decor/invalidation/selection-pseudo-with-decoration-invalidation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/invalidation/selection-pseudo-with-decoration-invalidation-002.html"
}
```

## style[0]

```css

    div {
        display: block;
        margin-top: 10px;
        margin-bottom: 10px;
        line-height: 20px;
        will-change: transform;
    }
    ::selection {
        background: yellow;
        color: currentColor;
        text-decoration-line: underline;
        text-decoration-style: line;
        text-decoration-thickness: 1px;
        text-underline-offset: 10px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
