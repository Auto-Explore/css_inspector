# css/css-cascade/layer-counter-style-override.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/layer-counter-style-override.html"
}
```

## style[0]

```css

#target, #reference {
  font-family: monospace;
  width: min-content;
}

#reference::before {
  content: '0000';
}

@counter-style three {
  system: cyclic;
  symbols: '000';
}

@counter-style four {
  system: cyclic;
  symbols: '0000';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
