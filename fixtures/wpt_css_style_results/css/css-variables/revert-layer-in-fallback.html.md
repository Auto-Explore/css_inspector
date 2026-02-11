# css/css-variables/revert-layer-in-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/revert-layer-in-fallback.html"
}
```

## style[0]

```css

@layer {
  #child {
    --x:PASS;
    margin: 1px;
    padding-left: 1px;
  }
}
@layer {
  #parent {
    --x:FAIL;
    margin: -1px;
    padding-left: -1px;
  }
  #child {
    --x: var(--unknown, revert-layer);
    margin: var(--unknown, revert-layer);
    padding-left: var(--unknown, revert-layer);
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
