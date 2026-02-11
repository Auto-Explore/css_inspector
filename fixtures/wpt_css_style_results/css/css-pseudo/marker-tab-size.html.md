# css/css-pseudo/marker-tab-size.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-tab-size.html"
}
```

## style[0]

```css

div {
  float: left;
  font: 25px/1 Ahem;
}
.inside {
  list-style-position: inside;
  width: 250px;
}
.outside {
  list-style-position: outside;
  width: 100px;
  margin-left: 150px;
}
ol {
  padding: 0;
}
.tab-size.explicit ::marker,
.tab-size.inherit {
  tab-size: 1;
}
.marker-string {
  list-style-type: "X\9p";
}
.marker-content::marker {
  content: "X\9p\9p";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
