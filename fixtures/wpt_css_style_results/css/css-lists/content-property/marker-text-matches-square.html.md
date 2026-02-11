# css/css-lists/content-property/marker-text-matches-square.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/content-property/marker-text-matches-square.html"
}
```

## style[0]

```css

/*
    Avoid using list-style:square directly, because the spec allows the
    rendering to deviate from the element's font.
    https://drafts.csswg.org/css-counter-styles-3/#simple-symbolic
*/
@counter-style my-square {
    system: extends square;
}
* {
    padding: 0;
    margin: 0;
}
ol {
    list-style: my-square inside;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “list-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
