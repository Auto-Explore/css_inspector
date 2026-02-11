# css/css-pseudo/marker-content-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-008-ref.html"
}
```

## style[0]

```css

li { list-style-type: "ab"; }
#t1 > li { list-style-type: "ab"; }
#t2 > li::before { content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAMCAIAAAD3UuoiAAAAGklEQVQoz2Nk%2BP%2BfgRqAiYFKYNSgUYOGp0EA%2BQMCFrJdTgsAAAAASUVORK5CYII%3D); margin-left: -24px; }
li::after {
  content: "d";
}
span { font-size: 32pt; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
