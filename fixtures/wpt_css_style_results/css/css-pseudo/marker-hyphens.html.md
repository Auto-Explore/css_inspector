# css/css-pseudo/marker-hyphens.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-hyphens.html"
}
```

## style[0]

```css

li {
  list-style-position: inside;
  width: 0;
}
::marker {
  white-space: normal;
}
.hyphens-manual.explicit ::marker,
.hyphens-manual.inherit {
  hyphens: manual;
}
.hyphens-none.explicit ::marker,
.hyphens-none.inherit {
  hyphens: none;
}
.marker-string {
  list-style-type: "foo\AD bar";
}
.marker-content::marker {
  content: "foo\AD bar";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
