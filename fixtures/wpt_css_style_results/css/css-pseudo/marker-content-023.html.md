# css/css-pseudo/marker-content-023.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-023.html"
}
```

## style[0]

```css

li {
  text-indent: 100px; /* Should not be inherited by ::marker */
}
::marker {
  text-indent: 100px; /* Should have no effect */
}
li > div {
  text-indent: 0;
}
.disc {
  list-style-type: disc;
}
.decimal {
  list-style-type: decimal;
}
.string {
  list-style-type: "3. ";
}
.content::marker {
  content: "4. ";
}
.rtl-marker ::marker {
  direction: rtl;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
