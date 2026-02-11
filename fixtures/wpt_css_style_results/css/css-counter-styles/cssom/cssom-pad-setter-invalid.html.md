# css/css-counter-styles/cssom/cssom-pad-setter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/cssom/cssom-pad-setter-invalid.html"
}
```

## style[0]

```css

@counter-style foo {
  system: extends decimal;
  pad: 3 '0';
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
