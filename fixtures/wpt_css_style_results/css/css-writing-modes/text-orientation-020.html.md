# css/css-writing-modes/text-orientation-020.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-020.html"
}
```

## style[0]

```css

.test ol {
  text-orientation: mixed; /*the default*/
}
.test ol li::marker {
  text-orientation: upright;
}
.ref ol {
  text-orientation: upright;
}
.ref span {
  text-orientation: mixed;
}
figure {
  writing-mode: vertical-rl;
  border: solid 1px black;
  padding: 1em 0 1ch;
  margin: 0 1ch;
  float: left;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
