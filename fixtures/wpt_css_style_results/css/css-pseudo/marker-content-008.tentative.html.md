# css/css-pseudo/marker-content-008.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-008.tentative.html"
}
```

## style[0]

```css

li {
  list-style-position: outside;
}
#t1 > li::marker { content: "a" "b"; }
#t2 > li::marker { content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAMCAIAAAD3UuoiAAAAGklEQVQoz2Nk%2BP%2BfgRqAiYFKYNSgUYOGp0EA%2BQMCFrJdTgsAAAAASUVORK5CYII%3D); }
li::before {
  content: "c";
}
li::after {
  content: "d";
}
span { font-size: 32pt; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
