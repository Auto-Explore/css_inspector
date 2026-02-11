# css/css-counter-styles/counter-style-at-rule/fallbacks-in-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/fallbacks-in-shadow-dom.html"
}
```

## style[0]

```css

@counter-style foo {
  system: cyclic;
  symbols: A B C;
}

@counter-style bar {
  system: fixed 4;
  symbols: D E F;
  fallback: foo;
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

## style[1]

```css

@counter-style foo {
  system: cyclic;
  symbols: X Y Z;
}

@counter-style baz {
  system: fixed 4;
  symbols: G H I;
  fallback: foo;
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
