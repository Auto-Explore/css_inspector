# css/css-properties-values-api/at-property-optional-initial-value.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/at-property-optional-initial-value.html"
}
```

## style[0]

```css

@property --registered {
    syntax: '*';
    initial-value: ;
    inherits: false;
}
#target {
    --test-bg: var(--registered) green;
    --test-fallback: var(--registered, red);
    background-color: var(--test-bg, var(--test-fallback));
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
