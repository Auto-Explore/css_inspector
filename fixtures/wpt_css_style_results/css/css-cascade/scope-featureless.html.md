# css/css-cascade/scope-featureless.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-featureless.html"
}
```

## style[0]

```css

:root {
  background: white;
  color: white;
}

:scope {
  --font-color: black;
}

#dut {
  color: var(--font-color);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
