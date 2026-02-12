# css/selectors/webkit-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/selectors/webkit-pseudo-element.html"
}
```

## style[0]

```css

#test {
  color: rgb(255, 0, 0);
}
span::-webkit-something-invalid, #test, ::-WeBkIt-sOmEtHiNg-NoNeXiSt123, ::-webkit-\ escaped {
  color: rgb(0, 255, 0);
}
::-webkitfoo, #test {
  color: rgb(255, 0, 0);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
