# css/css-properties-values-api/non-computed-unit-cycles.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/non-computed-unit-cycles.html"
}
```

## style[0]

```css

@property --registered-a {
  syntax: "*";
  inherits: false;
}

@property --registered-b {
  syntax: "*";
  inherits: false;
}

@property --registered-c {
  syntax: "*";
  inherits: false;
}

:root {
  --font-size-em: 2em;
  --line-height-lh: 2lh;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
