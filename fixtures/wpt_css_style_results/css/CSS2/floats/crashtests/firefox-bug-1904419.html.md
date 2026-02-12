# css/CSS2/floats/crashtests/firefox-bug-1904419.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/crashtests/firefox-bug-1904419.html"
}
```

## style[0]

```css

#a {
  columns: 15;
}
#b {
  grid-template: repeat(auto-fill, fit-content(3%)) / 1em;
}
*:only-of-type {
  white-space: pre-line;
  margin-bottom: 76%;
}
*:last-of-type {
  float: inline-end;
  display: flow;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
