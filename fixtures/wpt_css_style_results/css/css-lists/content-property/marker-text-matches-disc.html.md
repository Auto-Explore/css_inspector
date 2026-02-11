# css/css-lists/content-property/marker-text-matches-disc.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/content-property/marker-text-matches-disc.html"
}
```

## style[0]

```css

/*
    Avoid using list-style:disc directly, because the spec allows the
    rendering to deviate from the element's font.
    https://drafts.csswg.org/css-counter-styles-3/#simple-symbolic
*/
@counter-style my-disc {
    system: extends disc;
}
* {
    padding: 0;
    margin: 0;
}
ol {
    list-style: my-disc inside;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “list-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
