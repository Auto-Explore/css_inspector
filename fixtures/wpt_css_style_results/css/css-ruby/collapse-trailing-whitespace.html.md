# css/css-ruby/collapse-trailing-whitespace.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/collapse-trailing-whitespace.html"
}
```

## style[0]

```css

p {
  font: 20px/1 Ahem;
  xtext-align: justify;
  xtext-align-last: justify;
  width: 12em;
}
ruby { color: blue; }
rt { color: orange; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “xtext-align”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “xtext-align-last”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
