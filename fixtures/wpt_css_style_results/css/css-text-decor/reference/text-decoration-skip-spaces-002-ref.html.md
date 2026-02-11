# css/css-text-decor/reference/text-decoration-skip-spaces-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-skip-spaces-002-ref.html"
}
```

## style[0]

```css

span {
    /* Use separate longhands
       as Safari does not support the full syntax of the text-decoration shorthand
       at the time of writing,
       but that's not what we're testing here.
     */
    text-decoration: underline;
    text-decoration-color: blue;
}
div {
    color: orange;
    font-size: 2em;
    white-space: pre;
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
