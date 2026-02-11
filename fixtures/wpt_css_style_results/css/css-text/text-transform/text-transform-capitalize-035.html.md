# css/css-text/text-transform/text-transform-capitalize-035.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-transform/text-transform-capitalize-035.html"
}
```

## style[0]

```css

.test {
  text-transform: capitalize;
}
/* The remaining styles make it easier to review,
   but don't affect the behavior under test.
*/
.test {
  text-decoration-color: orange;
}
span {
    text-decoration: underline solid blue;
}
span + span::before {
    content: "\a";
    white-space:pre;
}
div {
    margin-top: 1em;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
