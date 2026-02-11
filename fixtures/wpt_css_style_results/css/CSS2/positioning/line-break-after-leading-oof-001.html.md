# css/CSS2/positioning/line-break-after-leading-oof-001.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/line-break-after-leading-oof-001.html"
}
```

## style[0]

```css

div {
  width: 5ch;
  /* When the line was broken after the leading OOF, it is hardly visible
     because it is an empty line box.
     Applying `text-indent` can make it visible; if the line is indented,
     it is the first line, proving the line did not wrap. */
  text-indent: 3ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
