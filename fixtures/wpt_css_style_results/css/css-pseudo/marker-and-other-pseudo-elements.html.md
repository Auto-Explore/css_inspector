# css/css-pseudo/marker-and-other-pseudo-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-and-other-pseudo-elements.html"
}
```

## style[0]

```css

li {
    color: red;
    font-size: 20px;
}

li::before {
    color: green;
    content: "PA";
}

li::after {
    color: green;
    content: "SSED if the list marker is green.";
}

li::marker {
    color: green;
}

li::first-letter {
    color: white;
    background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
