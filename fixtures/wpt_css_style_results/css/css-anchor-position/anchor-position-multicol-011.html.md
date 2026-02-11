# css/css-anchor-position/anchor-position-multicol-011.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-011.html"
}
```

## style[0]

```css

  #mc {
    columns: 2;
    gap: 0;
    display: list-item;
    list-style-position: outside;
    margin-left: 40px;
    width: 60px;
    color: red;
  }
  #mc::marker {
    /* anchor-name doesn't apply here. */
    anchor-name: --invalid;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
