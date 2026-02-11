# css/css-properties-values-api/registered-property-computation-color-004.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-computation-color-004.html"
}
```

## style[0]

```css

  @property --a {
    syntax: '<color>';
    inherits: true;
    initial-value: green;
  }

  body {
    --a: 1em;
  }

  div {
    width: 100px;
    height: 100px;
    background-color: var(--a);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
