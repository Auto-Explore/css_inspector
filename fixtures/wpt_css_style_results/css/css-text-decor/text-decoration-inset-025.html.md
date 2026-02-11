# css/css-text-decor/text-decoration-inset-025.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-inset-025.html"
}
```

## style[0]

```css

div > div {
  font: 20px/2 sans-serif;
  inline-size: fit-content;
  margin-bottom: 10px;
  border: 1px solid silver;
  color: silver;
  text-decoration: 3px blue overline;
  text-decoration-inset: -0.5em;
}
u {
  text-decoration: 3px green underline;
  text-decoration-inset: 2em;
}
.p1 {
  padding: 0 1em;
}
.m1 {
  margin: 0 1em;
}
.b1 {
  border: 1em solid transparent;
}
.p2 {
  padding: 0 0.5em;
}
.m2 {
  margin: 0 0.5em;
}
.b2 {
  border: 0.5em solid transparent;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-decoration-inset”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-decoration-inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
