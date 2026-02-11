# css/css-pseudo/marker-inherit-values.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-inherit-values.html"
}
```

## style[0]

```css

ol {
    color: red;
}

li { /* Originating element */
    color: green;
    font-family: sans-serif;
    font-size: x-large;
    font-style: italic;
    font-variant: small-caps;
    font-weight: bold;
    list-style-type: lower-alpha;
}

li::marker {
    color: inherit;
    font-family: inherit;
    font-size: inherit;
    font-style: inherit;
    font-variant: inherit;
    font-weight: inherit;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
