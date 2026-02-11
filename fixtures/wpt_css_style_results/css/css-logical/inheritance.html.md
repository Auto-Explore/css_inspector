# css/css-logical/inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-logical/inheritance.html"
}
```

## style[0]

```css

#reference-container {
  width: 300px;
}
#reference-container, #reference {
  border-style: solid; /* Avoid border-top-width computed style 0 */
  border-top-width: medium;
}

#container, #target {
  border-block-end-style: solid; /* Avoid border-block-end-width computed style 0 */
  border-block-start-style: solid;
  border-inline-end-style: solid;
  border-inline-start-style: solid;
}

#container {
  color: blue;
  width: 300px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
