# css/css-cascade/revert-layer-008.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-008.html"
}
```

## style[0]

```css

@layer revert-to, revert-from;

@layer revert-from {
  #target {
    font-size: 10px;
    transition: font-size 2s linear -1s;
  }

  #target.reverted {
    font-size: revert-layer;
  }
}

@layer revert-to {
  #target { font-size: 20px; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
