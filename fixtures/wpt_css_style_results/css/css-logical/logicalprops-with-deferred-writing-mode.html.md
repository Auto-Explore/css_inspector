# css/css-logical/logicalprops-with-deferred-writing-mode.html

```json
{
  "format_version": 3,
  "file": "css/css-logical/logicalprops-with-deferred-writing-mode.html"
}
```

## style[0]

```css

#parent {
  writing-mode: vertical-lr;
}

@layer {
  .revert-layer {
    writing-mode: vertical-rl;
  }
}
@layer {
  .revert-layer {
    writing-mode: horizontal-tb;
    writing-mode: revert-layer;
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
