# css/css-properties-values-api/crashtests/computed-property-universal-syntax.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/crashtests/computed-property-universal-syntax.html"
}
```

## style[0]

```css

@property --my-registered-property {
  syntax: "<color>";
  inherits: false;
  initial-value: #f0f2f5;
}

.outer {
  --unregistered-property: var(--my-registered-property);
}

.inner {
  --unregistered-property: #c6cfdb;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
