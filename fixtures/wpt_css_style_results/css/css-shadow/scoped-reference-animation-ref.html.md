# css/css-shadow/scoped-reference-animation-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/scoped-reference-animation-ref.html"
}
```

## style[0]

```css

@counter-style from-counter-style {
  system: cyclic;
  symbols: 'X';
}

@counter-style to-counter-style {
  system: cyclic;
  symbols: 'O';
}

#target1 {
  list-style-type: from-counter-style;
}

#target2 {
  list-style-type: to-counter-style;
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
