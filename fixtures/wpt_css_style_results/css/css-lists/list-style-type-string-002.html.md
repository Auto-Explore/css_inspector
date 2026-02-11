# css/css-lists/list-style-type-string-002.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/list-style-type-string-002.html"
}
```

## style[0]

```css

    .list { list-style-type: "" }
    .list > :nth-child(2) { list-style-type: "foo" }
    .list > :nth-child(3) { list-style-type: "foobar"; }
    .list > :nth-child(4) { list-style-type: "some very long text that is not going to fit and will overflow"; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
