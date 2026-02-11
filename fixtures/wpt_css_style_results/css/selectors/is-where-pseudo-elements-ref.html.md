# css/selectors/is-where-pseudo-elements-ref.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-pseudo-elements-ref.html"
}
```

## style[0]

```css

  #b::before, #d::before {
    content: "before ";
    color: green;
  }
  #e::after, #h::after {
    content: " after";
    color: green;
  }
  #a::first-letter, #b::first-letter {
    color: blue;
  }
  #g::first-line, #l::first-line {
    color: magenta;
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
