# css/css-conditional/container-queries/crashtests/focus-inside-content-visibility-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/focus-inside-content-visibility-crash.html"
}
```

## style[0]

```css

.spacer { height: 3000px; }
.auto { content-visibility: auto; }
#container {
  border: 1px solid black;
  width: 100px;
  height: 100px;

  container-type: size;
}
#input {
  width: 100%;
  visibility: hidden;
}
@container (min-width: 150px) {
  #input { visibility: visible; }
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
