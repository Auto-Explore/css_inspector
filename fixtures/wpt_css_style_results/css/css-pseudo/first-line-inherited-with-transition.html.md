# css/css-pseudo/first-line-inherited-with-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-line-inherited-with-transition.html"
}
```

## style[0]

```css

  p::first-line { color: orange; }
  span {
    background-color: black;
    transition: background-color 1000s steps(2, start);
  }
  span.lime {
    background-color: lime;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
