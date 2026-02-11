# css/CSS2/lists/list-style-020-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/list-style-020-ref.xht"
}
```

## style[0]

```css

/* For better readability */
.li {
  font-size: 1.5em;
  color: blue;
}
span {
  color: black;
  font-size: 0.67em;
  vertical-align: middle;
  padding-left: 0.2em;
}

.li {
  display: list-item;
  margin: 0.2em 0 0.2em 4em;
  border-left: solid thin orange;
}

.one {
  list-style-type: none;
  list-style-image: none;
  list-style-position: outside;
}
.two {
  list-style-type: none;
  list-style-image: none;
  list-style-position: outside;
}
.three {
  list-style-type: square;
  list-style-image: none;
  list-style-position: outside;
}
.four {
  list-style-type: square;
  list-style-image: none;
  list-style-position: outside;
}
.five {
  list-style-type: none;
  list-style-image: url(support/diamond.png);
  list-style-position: outside;
}
.six {
  list-style-type: none;
  list-style-image: url(support/diamond.png);
  list-style-position: outside;
}
.seven {
  list-style-type: disc;
  list-style-image: none;
  list-style-position: outside;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
