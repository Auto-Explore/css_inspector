# css/css-text-decor/invalidation/reference/selection-pseudo-with-decoration-invalidation-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/invalidation/reference/selection-pseudo-with-decoration-invalidation-002-ref.html"
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
        text-decoration-line: underline;
        text-decoration-style: line;
        text-decoration-thickness: 1px;
        text-underline-offset: 10px;
    }
    ::selection {
        background: yellow;
        color: currentColor;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
