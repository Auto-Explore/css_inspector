# css/css-transitions/custom-property-and-allow-discrete.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/custom-property-and-allow-discrete.html"
}
```

## style[0]

```css

  .target {
    transition: --foo 10s step-start;
    transition-behavior: allow-discrete;
    --foo: bar;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “transition-behavior”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
