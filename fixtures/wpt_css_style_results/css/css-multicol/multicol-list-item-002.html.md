# css/css-multicol/multicol-list-item-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-list-item-002.html"
}
```

## style[0]

```css

#self,
#before::before,
#after::after {
  display: list-item;
  list-style-type: decimal;
  list-style-position: inside;
  font: 25px/1 Ahem;
  width: 12ch;
  margin-top: 1ch;
}
#before::before,
#after::after {
  content: "X pXXXX XXpXX XXXXp";
}
.multicol #self,
.multicol #before::before,
.multicol #after::after {
  column-count: 2;
  column-gap: 2ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
