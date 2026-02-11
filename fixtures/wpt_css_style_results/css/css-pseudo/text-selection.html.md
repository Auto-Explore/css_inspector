# css/css-pseudo/text-selection.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/text-selection.html"
}
```

## style[0]

```css

.test {
  font: 10px/1 Ahem;
  margin-left: 200px;
}
#before::before {
  content: "::before";
  display: inline-block;
  margin-left: -80px;
}
#marker {
  display: list-item;
  list-style-type: "::marker";
}
#before-marker::before {
  content: "";
  display: list-item;
  list-style-type: "::marker";
  height: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
