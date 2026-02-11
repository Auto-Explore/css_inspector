# css/css-ruby/pseudo-first-line.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/pseudo-first-line.html"
}
```

## style[0]

```css

div::first-line {
  ruby-position: under;
  color: orange;
}
ruby::first-line, rt::first-line {
  color: red;
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
