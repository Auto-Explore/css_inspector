# css/css-values/typed_arithmetic_cycle.html

```json
{
  "format_version": 3,
  "file": "css/css-values/typed_arithmetic_cycle.html"
}
```

## style[0]

```css

@property --length {
  syntax: "<length>";
  initial-value: 0px;
  inherits: false;
}
:root {
  font-size: 228px;
}
div {
  --length: calc(10px * (2em / 1em));
  font-size: var(--length);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
