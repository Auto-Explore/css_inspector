# css/css-counter-styles/counter-style-at-rule/override-in-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/override-in-shadow-dom.html"
}
```

## style[0]

```css

@counter-style foo {
  system: fixed;
  symbols: A B C;
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
  system: fixed;
  symbols: D E F;
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

## style[2]

```css

@counter-style foo {
  system: fixed;
  symbols: G H I;
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
