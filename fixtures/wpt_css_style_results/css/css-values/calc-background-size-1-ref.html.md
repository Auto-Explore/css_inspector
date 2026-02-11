# css/css-values/calc-background-size-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-background-size-1-ref.html"
}
```

## style[0]

```css


p {
    height: 50px; width: 200px;
    border: thin solid;
    background-image: url(support/blue-32x32.png);
    background-repeat: no-repeat;
}

#one { background-size: 150px 20px; }
#two { background-image: none }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
