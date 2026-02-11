# css/css-lists/list-style-type-string-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/list-style-type-string-002-ref.html"
}
```

## style[0]

```css

    .list { list-style: none }
    .list > ::before {
      content: "";
      display: inline-block;
      width: 0px;
      direction: rtl;
      white-space: pre;
    }
    .list > :nth-child(2)::before { content: "foo" }
    .list > :nth-child(3)::before { content: "foobar"; }
    .list > :nth-child(4)::before { content: "some very long text that is not going to fit and will overflow"; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
